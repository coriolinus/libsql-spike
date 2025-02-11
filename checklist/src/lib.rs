use std::path::Path;

use libsql::{params, Connection, Database};
pub use libsql::{Cipher, EncryptionConfig};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{context}")]
    Libsql {
        context: &'static str,
        #[source]
        inner: libsql::Error,
    },
    #[error("this item is not present in the db; it may have been deleted")]
    MissingItem,
}

impl Error {
    pub(crate) fn libsql(context: &'static str) -> impl FnOnce(libsql::Error) -> Self {
        move |inner| Self::Libsql { context, inner }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Db {
    inner: Database,
}

impl Db {
    pub async fn new(path: impl AsRef<Path>, encryption_config: EncryptionConfig) -> Result<Self> {
        let inner = libsql::Builder::new_local(path)
            .encryption_config(encryption_config)
            .build()
            .await
            .map_err(Error::libsql("building local db connection"))?;

        let db = Self { inner };
        db.ensure_schema().await?;

        Ok(db)
    }

    pub(crate) fn conn(&self) -> Result<Connection> {
        self.inner
            .connect()
            .map_err(Error::libsql("establishing connection to db"))
    }

    async fn ensure_schema(&self) -> Result<()> {
        const SCHEMA: &str = include_str!("schema.sql");
        let conn = self.conn()?;

        for command in SCHEMA.split("\n\n") {
            conn.execute(command, ())
                .await
                .map_err(Error::libsql("executing schema"))?;
        }

        Ok(())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Constructor,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
    derive_more::FromStr,
)]
pub struct ChecklistId(i64);

pub struct Checklist {
    pub id: ChecklistId,
    pub name: String,
}

impl Checklist {
    pub async fn new(db: &Db, name: &str) -> Result<Self> {
        let conn = db.conn()?;

        let mut rows = conn
            .query(
                "INSERT INTO checklists(name) VALUES (?1) RETURNING id",
                [name],
            )
            .await
            .map_err(Error::libsql("creating checklist"))?;
        let row = rows
            .next()
            .await
            .map_err(Error::libsql("getting result row for creating checklist"))?
            .expect("insert query with RETURNING always produces at least one row");
        let id = row.get::<i64>(0).map_err(Error::libsql(
            "getting id from result row while creating checklist",
        ))?;
        let id = ChecklistId::new(id);

        Ok(Self {
            id,
            name: name.to_owned(),
        })
    }

    pub async fn load(db: &Db, id: ChecklistId) -> Result<Option<Self>> {
        let conn = db.conn()?;

        let mut rows = conn
            .query("SELECT name FROM checklists WHERE id = ?1", [*id])
            .await
            .map_err(Error::libsql("getting checklist by id"))?;
        let row = rows
            .next()
            .await
            .map_err(Error::libsql("getting result row for loading checklist"))?;

        row.map(|row| {
            let name = row.get_str(0).map_err(Error::libsql(
                "getting name from result row while getting checklist by id",
            ))?;

            Ok(Self {
                id,
                name: name.to_owned(),
            })
        })
        .transpose()
    }

    pub async fn all(db: &Db) -> Result<Vec<Self>> {
        let conn = db.conn()?;
        let mut checklists = Vec::new();

        let mut rows = conn
            .query("SELECT id, name FROM checklists", ())
            .await
            .map_err(Error::libsql("listing all checklists"))?;

        while let Some(row) = rows
            .next()
            .await
            .map_err(Error::libsql("getting next row while listing checklists"))?
        {
            let id = row.get::<i64>(0).map_err(Error::libsql(
                "getting id from result row while listing all checklists",
            ))?;
            let id = ChecklistId::new(id);
            let name = row
                .get_str(1)
                .map_err(Error::libsql(
                    "getting name from result row while listing all checklists",
                ))?
                .to_owned();
            checklists.push(Self { id, name });
        }

        Ok(checklists)
    }

    pub async fn delete(db: &Db, id: ChecklistId) -> Result<()> {
        let conn = db.conn()?;

        conn.execute("DELETE FROM checklists WHERE id = ?1", [*id])
            .await
            .map_err(Error::libsql("deleting checklist"))?;

        Ok(())
    }

    pub async fn items(&self, db: &Db) -> Result<Vec<Item>> {
        let conn = db.conn()?;
        let mut items = Vec::new();

        let mut rows = conn
            .query(
                "SELECT id, item FROM items WHERE checklist = ?1",
                [*self.id],
            )
            .await
            .map_err(Error::libsql("selecting items for checklist"))?;

        while let Some(row) = rows.next().await.map_err(Error::libsql(
            "getting next row while listing items for a checklist",
        ))? {
            let id = row.get::<i64>(0).map_err(Error::libsql(
                "getting id from result row while listing checklist items",
            ))?;
            let id = ItemId::new(id);
            let item = row
                .get_str(1)
                .map_err(Error::libsql(
                    "getting item from result row while listing checklist items",
                ))?
                .to_owned();
            items.push(Item {
                id,
                checklist: self.id,
                item,
            });
        }

        Ok(items)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Constructor,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
    derive_more::FromStr,
)]
pub struct ItemId(i64);

pub struct Item {
    pub id: ItemId,
    pub checklist: ChecklistId,
    pub item: String,
}

impl Item {
    pub async fn new(db: &Db, checklist: ChecklistId, item: String) -> Result<Self> {
        let conn = db.conn()?;

        let mut rows = conn
            .query(
                "INSERT INTO items(checklist, item) VALUES (?1, ?2) RETURNING id",
                params!(*checklist, item.clone()),
            )
            .await
            .map_err(Error::libsql("creating item"))?;
        let row = rows
            .next()
            .await
            .map_err(Error::libsql("getting result row for creating item"))?
            .expect("insert query with RETURNING always produces at least one row");
        let id = row.get::<i64>(0).map_err(Error::libsql(
            "getting id from result row while creating item",
        ))?;
        let id = ItemId::new(id);

        Ok(Self {
            id,
            checklist,
            item,
        })
    }

    pub async fn load(db: &Db, id: ItemId) -> Result<Option<Self>> {
        let conn = db.conn()?;

        let mut rows = conn
            .query("SELECT checklist, item FROM items WHERE id = ?1", [*id])
            .await
            .map_err(Error::libsql("getting item by id"))?;
        let row = rows
            .next()
            .await
            .map_err(Error::libsql("getting result row for loading checklist"))?;

        row.map(|row| {
            let checklist = row
                .get::<i64>(0)
                .map_err(Error::libsql(
                    "getting checklist from result row while getting item by id",
                ))?
                .into();
            let item = row
                .get_str(1)
                .map_err(Error::libsql(
                    "getting name from result row while getting item by id",
                ))?
                .to_owned();

            Ok(Self {
                id,
                checklist,
                item,
            })
        })
        .transpose()
    }

    pub async fn delete(db: &Db, id: ItemId) -> Result<()> {
        let conn = db.conn()?;

        conn.execute("DELETE FROM items WHERE id = ?1", [*id])
            .await
            .map_err(Error::libsql("deleting item"))?;

        Ok(())
    }

    pub async fn is_set(&self, db: &Db) -> Result<bool> {
        let conn = db.conn()?;

        let mut rows = conn
            .query("SELECT checked FROM items WHERE id = ?1", [*self.id])
            .await
            .map_err(Error::libsql("getting item by id"))?;
        let row = rows
            .next()
            .await
            .map_err(Error::libsql("getting result row for loading checklist"))?
            .ok_or(Error::MissingItem)?;

        row.get::<bool>(0)
            .map_err(Error::libsql("getting checked status from result row"))
    }

    pub async fn set_checked(&self, db: &Db, checked: bool) -> Result<()> {
        let conn = db.conn()?;

        let rows = conn
            .execute(
                "UPDATE items SET checked = ?1 WHERE id = ?2",
                params!(checked, *self.id),
            )
            .await
            .map_err(Error::libsql("updating checked status for item"))?;

        if rows == 0 {
            Err(Error::MissingItem)
        } else {
            Ok(())
        }
    }
}
