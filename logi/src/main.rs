mod cli;
mod journal;
mod storage;
mod auth;

fn main() {
    // let cli = Cli::parse();

    // match cli.command {
    //     cli::Command::Write { text, date } => {
    //         journal::write_entry(&text, date.as_deref()).unwrap();
    //     }
    //     cli::Command::View { date } => {
    //         journal::view_entry(date.as_deref()).unwrap();
    //     }
    // }

    match auth::get_google_token() {
        Ok(token) => println!("✅ Access Token: {}", token),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
