use std::fs;
use std::io;
use gnome::{initialize_directories, get_desktop_dir, is_image_file, is_screenshot, is_pdf_file, move_file};

fn main() -> io::Result<()> {
    if let Some(desktop_dir) = get_desktop_dir() {
        let (images_dir, screenshots_dir, pdfs_dir) = initialize_directories(&desktop_dir);
        let mut images_count = 0;
        let mut screenshots_count = 0;
        let mut pdfs_count = 0;

        for entry in fs::read_dir(&desktop_dir)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() {
                if is_image_file(&file_path) {
                    let mut dest_dir = &images_dir;
                    images_count += 1;

                    if is_screenshot(&file_path) {
                        dest_dir = &screenshots_dir;
                        screenshots_count += 1;
                        images_count -= 1;
                    }

                    move_file(&file_path, dest_dir)?;
                }

                if is_pdf_file(&file_path) {
                    pdfs_count += 1;
                    move_file(&file_path, &pdfs_dir)?;
                }
            }
        }

        println!("Gnome has finished cleaning up the desktop. Moved {} images, {} screenshots and {} pdfs.", images_count, screenshots_count, pdfs_count);
    } else {
        println!("Gnome could not find the desktop :(.");
    }

    Ok(())
}
