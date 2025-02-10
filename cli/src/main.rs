mod cli;
use anyhow::Context;
use bytes::Bytes;
use checklist::{Checklist, Cipher, Db, EncryptionConfig, Item};
use clap::Parser as _;
use cli::{
    Cli, ItemVerb, ItemVerbAction, ListVerb, ListVerbAction, NewChecklist, NewItem,
    RemoveChecklist, RemoveItem, ShowAllChecklists, ShowAllItems, ToggleItem,
};
use color_print::cprintln;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let path = dirs::data_local_dir()
        .context("data local dir must exist on this system")?
        .join("checklist/db.sqlite3");
    let parent = path.parent().expect("the path above always has a parent");
    std::fs::create_dir_all(parent).context("creating checklist data directory")?;

    // Very Secure
    let device = whoami::devicename();
    let arch = whoami::arch();
    let username = whoami::username();
    let key = format!("{device}-{arch}-{username}");
    let encryption_config = EncryptionConfig {
        cipher: Cipher::Aes256Cbc,
        encryption_key: Bytes::from(key),
    };

    let db = Db::new(path, encryption_config)
        .await
        .context("connecting to database")?;

    match cli.noun {
        cli::Noun::List(ListVerbAction {
            verb: ListVerb::ShowAll(ShowAllChecklists {}),
        }) => {
            for checklist in Checklist::all(&db).await.context("getting checklists")? {
                show_checklist(&checklist);
            }
        }
        cli::Noun::List(ListVerbAction {
            verb: ListVerb::New(NewChecklist { name }),
        }) => {
            let checklist = Checklist::create(&db, &name)
                .await
                .context("creating checklist")?;
            show_checklist(&checklist);
        }
        cli::Noun::List(ListVerbAction {
            verb: ListVerb::Remove(RemoveChecklist { id }),
        }) => {
            Checklist::delete(&db, id)
                .await
                .context("deleting checklist")?;
        }
        cli::Noun::Item(ItemVerbAction {
            verb:
                ItemVerb::ShowAll(ShowAllItems {
                    checklist_id,
                    omit_header,
                }),
        }) => {
            let checklist = Checklist::load(&db, checklist_id)
                .await
                .context("getting checklist")?
                .context("checklist not found")?;

            if !omit_header {
                show_checklist(&checklist);
                println!("=========================")
            }

            for item in checklist.items(&db).await.context("getting items")? {
                let checked = item.is_set(&db).await.context("getting item status")?;
                show_item(&item, checked);
            }
        }
        cli::Noun::Item(ItemVerbAction {
            verb: ItemVerb::New(NewItem { checklist_id, name }),
        }) => {
            let item = Item::new(&db, checklist_id, name)
                .await
                .context("creating item")?;
            show_item(&item, false);
        }
        cli::Noun::Item(ItemVerbAction {
            verb: ItemVerb::Remove(RemoveItem { id }),
        }) => {
            Item::delete(&db, id).await.context("deleting item")?;
        }
        cli::Noun::Item(ItemVerbAction {
            verb: ItemVerb::Toggle(ToggleItem { id }),
        }) => {
            let item = Item::load(&db, id)
                .await
                .context("loading item from db")?
                .context("item not found")?;
            let checked = item
                .is_set(&db)
                .await
                .context("getting item check status")?;
            item.set_checked(&db, !checked)
                .await
                .context("updating item check status")?;
            show_item(&item, !checked);
        }
    }

    Ok(())
}

fn show_checklist(Checklist { id, name }: &Checklist) {
    cprintln!("<dim>{id:>6}:</dim> {name}")
}

fn show_item(Item { id, item, .. }: &Item, checked: bool) {
    if checked {
        cprintln!("<dim>{id:>6}:</dim> ☑ <strike>{item}</strike>");
    } else {
        cprintln!("<dim>{id:>6}:</dim> ☐ {item}");
    }
}
