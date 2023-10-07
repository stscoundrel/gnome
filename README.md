# Gnome

A housekeeper gnome to sort through your messy desktop.

Essentially just cleanup for your overflowing image files, screenshots and docs. Moves all to their own subfolders, keeping your desktop clean.

# Build executables with Docker

`docker build -t gnome .`

`docker run -it --rm -v "$(pwd)/dist":/dist gnome`

Executables will be compiled in /dist/ folder.

## Usage

- Run the script anywhere.
- Desktop location will be automatically detected & cleaned of image files.
- Example output: `Gnome has finished cleaning up the desktop. Moved 76 images, 362 screenshots and 6 documents.`.

## Why though?

I might've cluttered couple of different machines with hundreds of images on their respective desktops.
