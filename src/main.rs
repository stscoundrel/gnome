use std::fs;
use std::io;
use gnome::{initialize_directories, get_desktop_dir, is_image_file, is_screenshot, is_document, move_file};

fn main() -> io::Result<()> {
    if let Some(desktop_dir) = get_desktop_dir() {
        let (images_dir, screenshots_dir, pdfs_dir) = initialize_directories(&desktop_dir);
        let mut images_count = 0;
        let mut screenshots_count = 0;
        let mut documents_count = 0;

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

                if is_document(&file_path) {
                    documents_count += 1;
                    move_file(&file_path, &pdfs_dir)?;
                }
            }
        }

        println!("Gnome has finished cleaning up the desktop. Moved {} images, {} screenshots and {} documents.", images_count, screenshots_count, documents_count);
    } else {
        println!("Gnome could not find the desktop :(.");
    }

    // Any key input to exit.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");    

    Ok(())
}
