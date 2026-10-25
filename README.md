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

## Installation

Clone the repository:

```bash
git clone https://github.com/Thisguyshaaa11/RustDL.git
cd RustDL
```

Build the application:

```bash
cargo build --release
```

Run RustDL:

```bash
./target/release/belajar-rust
```

## Notes

- Some videos may be unavailable because of privacy settings, regional restrictions, or platform changes.
- Only download content you have permission to download.
- Conversion depends on FFmpeg and the codecs available in your environment.

## License

See the [LICENSE](LICENSE) file.

