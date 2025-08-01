use directories::ProjectDirs;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::{PathBuf};

pub fn get_data_dir() -> io::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "logi", "logi") {
        let dir = proj_dirs.data_dir().join("entries");
        create_dir_all(&dir)?; // 디렉토리가 없으면 생성
        Ok(dir)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Could not find data directory"))
    }
}

pub fn get_entry_file_path(date: &str) -> io::Result<PathBuf> {
    let dir = get_data_dir()?;
    Ok(dir.join(format!("{date}.md")))
}

pub fn write_to_file(path: &PathBuf, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn read_from_file(path: &PathBuf) -> io::Result<String> {
    std::fs::read_to_string(path)
}
