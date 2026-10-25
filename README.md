# 🦀 RustDL

**RustDL** is a simple interactive command-line video downloader written in Rust.

It uses [yt-dlp](https://github.com/yt-dlp/yt-dlp) to download videos and FFmpeg to convert supported downloads to MP4.

## Features

- Interactive terminal menu
- Selectable video quality: 144p, 240p, 360p, 480p, 720p, and 1080p
- Best available quality option
- Downloads saved to `~/Downloads`
- MP4 conversion using H.264 video and AAC audio

## Requirements

Install these tools before running RustDL:

- Rust and Cargo
- Git
- yt-dlp
- Deno
- FFmpeg

## Linux Dependencies

RustDL requires the following software:

- **Git**: to clone the repository
- **Rust and Cargo**: to build RustDL
- **yt-dlp**: to download videos
- **Deno**: for JavaScript runtime support in yt-dlp
- **FFmpeg**: for video conversion

### Debian / Ubuntu

Install Git, FFmpeg, and Deno:

```bash
sudo apt update
sudo apt install git ffmpeg curl unzip
```

Install Rust using the official Rust installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the installer instructions, then restart your terminal or run:

```bash
source "$HOME/.cargo/env"
```

Install Deno using its official installer:

```bash
curl -fsSL https://deno.land/install.sh | sh
```

Make sure `~/.deno/bin` is in your PATH.

Install yt-dlp using the official installation instructions:
https://github.com/yt-dlp/yt-dlp/wiki/Installation

### Build and Run

```bash
git clone https://github.com/Thisguyshaaa11/RustDL.git
cd RustDL
cargo build --release
./target/release/belajar-rust
```

### Notes

RustDL depends on external tools and supported video platforms. Some videos may be unavailable because of login requirements, privacy settings, regional restrictions, or platform changes.

## License

See the [LICENSE](LICENSE) file.

