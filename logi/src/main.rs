mod cli;
mod journal;
mod storage;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Command::Write { text, date } => {
            journal::write_entry(&text, date.as_deref()).unwrap();
        }
        cli::Command::View { date } => {
            journal::view_entry(date.as_deref()).unwrap();
        }
    }
}
