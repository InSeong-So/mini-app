use std::fs::File;
use std::io::{Read, Cursor};
use std::path::PathBuf;

use reqwest::blocking::{Client, multipart};
use serde_json::json;

pub fn upload_to_drive(file_path: PathBuf, access_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 📂 파일 열기
    let mut file = File::open(&file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

    // 📄 메타데이터 JSON → 스트링으로 인코딩
    let metadata_json = json!({
        "name": file_name,
        "mimeType": "application/vnd.google-apps.document" // Google Docs 형식
    });
    let metadata_str = metadata_json.to_string();

    // 📦 multipart: metadata (as JSON) + file
    let metadata_part = multipart::Part::text(metadata_str)
        .mime_str("application/json; charset=UTF-8")?;

    let file_part = multipart::Part::reader(Cursor::new(file_contents.into_bytes()))
        .file_name(file_name.clone())
        .mime_str("text/plain")?;

    let form = multipart::Form::new()
        .part("metadata", metadata_part)
        .part("file", file_part);

    let client = Client::new();

    let res = client.post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .bearer_auth(access_token)
        .multipart(form)
        .send()?;

    if res.status().is_success() {
        println!("✅ File '{}' uploaded to Google Drive!", file_name);
    } else {
        let text = res.text()?;
        eprintln!("❌ Upload failed: {}", text);
    }

    Ok(())
}
