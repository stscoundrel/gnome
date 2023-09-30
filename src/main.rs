use std::fs;
use std::io;
use gnome::{initialize_directories, get_desktop_dir, is_image_file, is_screenshot};

fn main() -> io::Result<()> {
    if let Some(desktop_dir) = get_desktop_dir() {
        let (images_dir, screenshots_dir) = initialize_directories(&desktop_dir);
        let mut images_count = 0;
        let mut screenshots_count = 0;

        for entry in fs::read_dir(&desktop_dir)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() && is_image_file(&file_path) {
                let mut dest_path = images_dir.join(entry.file_name());
                images_count += 1;

                if is_screenshot(&file_path) {
                    dest_path = screenshots_dir.join(entry.file_name());
                    screenshots_count += 1;
                    images_count -= 1;
                }

                fs::rename(&file_path, &dest_path)?;
            }
        }

        println!("Gnome has finished cleaning up the desktop. Moved {} images and {} screenshots", images_count, screenshots_count);
    } else {
        println!("Gnome could not find the desktop :(.");
    }

    Ok(())
}
