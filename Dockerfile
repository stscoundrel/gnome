# Use a Rust build image.
FROM rust AS builder

# Set the working directory.
WORKDIR /app

# Install build dependencies for cross-compiling to Windows.
RUN apt-get update && \
    apt-get install -y mingw-w64

RUN rustup target add x86_64-pc-windows-gnu

# Copy the entire project into the container.
COPY . .

# Build the Rust application for Windows.
RUN cargo build --release --target=x86_64-pc-windows-gnu

# Create a dist folder.
RUN mkdir /dist

# Copy the built Windows executable to the dist folder.
CMD cp /app/target/x86_64-pc-windows-gnu/release/gnome.exe /dist/gnome.exe