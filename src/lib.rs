use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn initialize_directories(desktop_dir: &Path) -> (PathBuf, PathBuf) {
    // Create destination directories, if they're missing.
    let screenshors_dir = desktop_dir.join("gnome-screenshots");
    let images_dir = desktop_dir.join("gnome-images");

    fs::create_dir_all(&screenshors_dir).unwrap();
    fs::create_dir_all(&images_dir).unwrap();

    (images_dir, screenshors_dir)
}

pub fn is_image_file(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension() {
        matches!(ext.to_str(), Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("webp"))
    } else {
        false
    }
}

pub fn is_screenshot(file_path: &Path) -> bool {
    if let Some(file_name) = file_path.file_name() {
        if let Some(name) = file_name.to_str() {
            name.starts_with("Screenshot")
        } else {
            false
        }
    } else {
        false
    }
}

pub fn get_desktop_dir() -> Option<PathBuf> {
    dirs::desktop_dir()
}

pub fn move_file(file_path: &Path, dest_dir: &Path) -> Result<(), Error> {
    // Gracefully moves files without overwrites or samename losses.
    let mut new_path = dest_dir.join(file_path.file_name().unwrap());
    print!("{:?}", new_path);

    let mut counter = 1;
    while new_path.exists() {
        let new_file_name = format!(
            "{} ({}).{}",
            file_path.file_stem().unwrap().to_string_lossy(),
            counter,
            file_path.extension().unwrap().to_string_lossy()
        );
        new_path = dest_dir.join(new_file_name);
        counter += 1;
    }

    fs::rename(file_path, new_path)
}