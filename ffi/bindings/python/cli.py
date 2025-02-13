# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "click",
#     "xdg-base-dirs",
# ]
# ///

import asyncio
import checklist_ffi
import click
import os
from pathlib import Path

from xdg_base_dirs import xdg_data_home


@click.group
@click.option(
    "-p",
    "--path",
    help="Path to database",
    type=click.Path(dir_okay=False, readable=True, writable=True, path_type=Path),
)
@click.option(
    "-E",
    "--encryption-key-file",
    help="Path to file containing encryption key for data at rest",
)
@click.option(
    "-e",
    "--encryption-key",
    help="Encryption key for data at rest; default '$USER@$NAME'",
)
@click.pass_context
def cli(ctx, path: str, encryption_key_file: str | None, encryption_key: str | None):
    if path is None:
        path = xdg_data_home() / "checklist/db.sqlite3"

    path.resolve()
    if path.parent is not None:
        path.parent.mkdir(parents=True, exist_ok=True)
    path = str(path)

    key = None
    if encryption_key_file is not None:
        with open(encryption_key_file, "r") as fd:
            key = fd.read()

    if key is None and encryption_key is not None:
        key = encryption_key.encode()

    if key is None:
        user = os.environ.get("USER") or ""
        hostname = os.environ.get("NAME") or ""
        key = f"{user}@{hostname}".encode()

    ctx.obj = asyncio.run(checklist_ffi.db_new(path, key))


@cli.command()
@click.argument("name", nargs=-1)
@click.pass_context
def create_checklist(ctx, name: tuple[str, ...]):
    """
    Create a checklist named NAME.

    Emits the checklist ID (CK_ID).
    """
    name = " ".join(name)
    db = ctx.find_object(checklist_ffi.Db)
    checklist = asyncio.run(checklist_ffi.checklist_new(db, name))
    click.echo(checklist.id())


@cli.command()
@click.argument("ck_id", type=int)
@click.argument("item", nargs=-1)
@click.pass_context
def create_item(ctx, ck_id: int, item: tuple[str, ...]):
    """
    Creates a checklist item.

    CK_ID is the id of the checklist to which this item will be attached.
    ITEM is the checklist item; an arbitrary string.

    Emits the item ID (ITEM_ID). All items have a unique ID across checklists.
    """
    item = " ".join(item)
    db = ctx.find_object(checklist_ffi.Db)
    item = asyncio.run(checklist_ffi.item_new(db, ck_id, item))
    click.echo(item.id())


@cli.command()
@click.argument("item_id", type=int)
@click.pass_context
def toggle_item(ctx, item_id: int):
    """
    Toggles a checklist item by its id.

    Emits 1 if the item is now checked, or 0 if it is now unchecked.
    """

    async def toggle(db: checklist_ffi.Db, item_id: int) -> bool:
        item = await checklist_ffi.item_load(db, item_id)
        state = await item.is_set(db)
        state = not state
        await item.set_checked(db, state)
        return state

    db = ctx.find_object(checklist_ffi.Db)
    state = asyncio.run(toggle(db, item_id))
    click.echo(f"{int(state)}")


if __name__ == "__main__":
    cli()
