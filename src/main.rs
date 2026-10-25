
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Gagal menampilkan prompt");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Gagal membaca input");

    value.trim().to_string()
}

fn banner() {
    println!(
        r#"
 ____            _   ____  _
|  _ \ _   _ ___| |_|  _ \| |
| |_) | | | / __| __| | | | |
|  _ <| |_| \__ \ |_| |_| | |___
|_| \_\\__,_|___/\__|____/|_____|

       R U S T D L   D O W N L O A D E R
       YouTube | TikTok | Instagram | Facebook
"#
    );
}

fn download_video(url: &str, quality: &str) {
    let home = match env::var("HOME") {
        Ok(value) => PathBuf::from(value),
        Err(_) => {
            eprintln!("Error: variabel HOME tidak ditemukan.");
            return;
        }
    };

    let downloads = home.join("Downloads");
    let deno = home.join(".deno/bin/deno");

    if let Err(error) = fs::create_dir_all(&downloads) {
        eprintln!("Gagal membuat folder Downloads: {error}");
        return;
    }

    let format = if quality == "best" {
        "bestvideo*+bestaudio/best".to_string()
    } else {
        format!(
            "bestvideo[height<={0}]+bestaudio/best[height<={0}]/best",
            quality
        )
    };

    println!("\nMemulai download...");
    println!("URL     : {url}");
    println!("Kualitas: {}\n", if quality == "best" {
        "Terbaik"
    } else {
        quality
    });

    let status = Command::new("yt-dlp")
        .arg("--no-playlist")
        .arg("--js-runtimes")
        .arg(format!("deno:{}", deno.display()))
        .arg("-f")
        .arg(&format)
        .arg("--recode-video")
        .arg("mp4")
        .arg("--postprocessor-args")
        .arg("VideoConvertor+ffmpeg_o:-c:v libx264 -preset ultrafast -crf 28 -pix_fmt yuv420p -c:a aac -b:a 96k -movflags +faststart")
        .arg("-P")
        .arg(&downloads)
        .arg("-o")
        .arg("%(title).180B [%(id)s].%(ext)s")
        .arg(url)
        .status();

    match status {
        Ok(result) if result.success() => {
            println!("\nDownload selesai atau file sudah tersedia.");
            println!("Lokasi: {}", downloads.display());
            println!("Catatan: cek codec video jika file lama tidak dikonversi.");
        }
        Ok(result) => {
            eprintln!("\nDownload gagal dengan kode: {}", result);
            eprintln!("Periksa URL, koneksi internet, dan versi yt-dlp.");
        }
        Err(error) => {
            eprintln!("\nGagal menjalankan yt-dlp: {error}");
            eprintln!("Pastikan yt-dlp, Deno, dan FFmpeg sudah terpasang.");
        }
    }
}

fn main() {
    loop {
        println!("\x1B[2J\x1B[H");
        banner();

        println!("  [1] Download video");
        println!("  [2] Buka folder Downloads");
        println!("  [3] Keluar");
        println!();

        let choice = input("  RustDL > ");

        match choice.as_str() {
            "1" => {
                let url = input("\n  Masukkan URL video: ");

                if url.is_empty() {
                    println!("URL tidak boleh kosong.");
                    let _ = input("\nTekan Enter untuk kembali...");
                    continue;
                }

                println!("\n  Pilih kualitas video:");
                println!("  [1] 144p");
                println!("  [2] 240p");
                println!("  [3] 360p");
                println!("  [4] 480p");
                println!("  [5] 720p");
                println!("  [6] 1080p");
                println!("  [7] Terbaik");
                println!();

                let quality_choice = input("  Kualitas > ");

                let quality = match quality_choice.as_str() {
                    "1" => "144",
                    "2" => "240",
                    "3" => "360",
                    "4" => "480",
                    "5" => "720",
                    "6" => "1080",
                    "7" => "best",
                    _ => {
                        println!("Pilihan kualitas tidak valid.");
                        let _ = input("\nTekan Enter untuk kembali...");
                        continue;
                    }
                };

                download_video(&url, quality);
                let _ = input("\nTekan Enter untuk kembali ke menu...");
            }
            "2" => {
                let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
                let downloads = PathBuf::from(home).join("Downloads");

                if let Err(error) = fs::create_dir_all(&downloads) {
                    eprintln!("Gagal membuka folder: {error}");
                } else {
                    let result = Command::new("xdg-open")
                        .arg(&downloads)
                        .status();

                    if let Err(error) = result {
                        eprintln!("Gagal membuka file manager: {error}");
                        println!("Lokasi folder: {}", downloads.display());
                    }
                }

                let _ = input("\nTekan Enter untuk kembali...");
            }
            "3" => {
                println!("\n  RustDL ditutup. Sampai jumpa!\n");
                break;
            }
            _ => {
                println!("Pilihan tidak valid.");
                let _ = input("\nTekan Enter untuk kembali...");
            }
        }
    }
}

