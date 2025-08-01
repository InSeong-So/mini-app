use chrono::{Local, NaiveDate};
use colored::*;
use std::io;

use crate::storage::{get_entry_file_path, read_from_file, write_to_file};

pub fn write_entry(text: &str, date_opt: Option<&str>) -> io::Result<()> {
    let date_str = match date_opt {
        Some(d) => d.to_string(),
        None => Local::now().format("%Y-%m-%d").to_string(),
    };

    let path = get_entry_file_path(&date_str)?;
    write_to_file(&path, text)?;
    println!("✍️  Entry for {} saved to:\n{}", date_str.green(), path.display());
    Ok(())
}

pub fn view_entry(date_opt: Option<&str>) -> io::Result<()> {
    let date_str = match date_opt {
        Some(d) => d.to_string(),
        None => Local::now().format("%Y-%m-%d").to_string(),
    };

    let path = get_entry_file_path(&date_str)?;
    match read_from_file(&path) {
        Ok(content) => {
            println!("📖 Journal Entry for {}\n{}", date_str.blue().bold(), "-".repeat(40));
            println!("{}", content);
        }
        Err(_) => {
            println!("⚠️  No entry found for {}", date_str.red());
        }
    }
    Ok(())
}
