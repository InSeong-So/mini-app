use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "logi", version = "0.1.0", about = "Simple journaling CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Write a journal entry
    Write {
        #[arg(short, long)]
        text: String,
        #[arg(short, long)]
        date: Option<String>, // yyyy-mm-dd
    },

    /// View a journal entry
    View {
        #[arg(short, long)]
        date: Option<String>,
    },
}
