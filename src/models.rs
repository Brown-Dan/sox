use serde::Serialize;

#[derive(
    clap::ValueEnum, Clone, Debug, Serialize,
)]
pub enum Command {
    Add,
    List,
    Remove,
    Update,
    Rename,
    Search,
}