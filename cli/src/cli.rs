use checklist::{ChecklistId, ItemId};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub noun: Noun,
}

#[derive(Debug, Subcommand)]
pub enum Noun {
    /// Manage lists
    List(ListVerbAction),

    /// Manage items
    Item(ItemVerbAction),
}

#[derive(Debug, Args)]
pub struct ListVerbAction {
    #[command(subcommand)]
    pub verb: ListVerb,
}

#[derive(Debug, Subcommand)]
pub enum ListVerb {
    /// Show all checklists
    ShowAll(ShowAllChecklists),

    /// Create a new checklist
    New(NewChecklist),

    /// Delete a checklist
    Remove(RemoveChecklist),
}

#[derive(Debug, Args)]
pub struct ShowAllChecklists {}

#[derive(Debug, Args)]
pub struct NewChecklist {
    /// Name of this checklist
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RemoveChecklist {
    /// Id of the checklist to remove
    pub id: ChecklistId,
}

#[derive(Debug, Args)]
pub struct ItemVerbAction {
    #[command(subcommand)]
    pub verb: ItemVerb,
}

#[derive(Debug, Subcommand)]
pub enum ItemVerb {
    /// Show all items in a checklist
    ShowAll(ShowAllItems),

    /// Create a new item in a checklist
    New(NewItem),

    /// Delete an item in a checklist
    Remove(RemoveItem),

    /// Toggle completion status of an item in a checklist
    Toggle(ToggleItem),
}

#[derive(Debug, Args)]
pub struct ShowAllItems {
    /// Checklist Id for items to show
    pub checklist_id: ChecklistId,

    /// When set, omit the item header
    #[arg(short, long)]
    pub omit_header: bool,
}

#[derive(Debug, Args)]
pub struct NewItem {
    /// Checklist Id in which to put this item
    pub checklist_id: ChecklistId,

    /// Name of this item
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RemoveItem {
    /// Id of the item to remove
    pub id: ItemId,
}

#[derive(Debug, Args)]
pub struct ToggleItem {
    /// Id of the item to toggle
    pub id: ItemId,
}
