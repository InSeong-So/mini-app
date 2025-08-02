mod auth;
mod upload;

fn main() {
    match auth::get_google_token() {
        Ok(token) => {
            println!("✅ Got token, uploading...");
            let path = std::path::PathBuf::from("/Users/tene/Development/toys/mini-app/logi/2025-08-01.md");
            match upload::upload_to_drive(path, &token) {
                Ok(_) => println!("📁 Upload success!"),
                Err(e) => eprintln!("❌ Upload error: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Auth error: {}", e),
    }
}
