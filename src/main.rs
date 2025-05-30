mod archarchive;

use std::fs;
use std::io::Write;
use anyhow::{Result, anyhow};
use tokio::process::Command;
use archarchive::{ArchArchive, parser};
use crate::archarchive::menu::detect_language;

#[tokio::main]
async fn main() -> Result<()> {
    let lang = detect_language();
    let mut aa = ArchArchive::init();
    match aa.menu_run().await {
        Ok(url) => {
            println!("URL: {}", url);

            println!("{}", match lang.as_str() {
                "ru" => "Создаю резервную копию mirrorlist...",
                _ => "Creating a mirrorlist backup...",
            });
            fs::rename("/etc/pacman.d/mirrorlist", "/etc/pacman.d/mirrorlist.bak")?;

            println!("{}", match lang.as_str() {
                "ru" => "Записываю зеркало archarchive в mirrorlist...",
                _ => "Writing an archarchive mirror to mirrorlist..."
            });
            let mut file = fs::File::create("/etc/pacman.d/mirrorlist")?;
            file.write_all(format!("Server = {url}").as_bytes())?;
            
            println!("{}", match lang.as_str() {
                "ru" => "Обновляю пакеты через pacman...",
                _ => "Updating packages via pacman...",
            });

            Command::new("sudo")
                .args(["pacman", "-Syyuu"])
                .status()
                .await?;

            println!("{}", match lang.as_str() {
                "ru" => "Восстанавливаю mirrorlist из резервной копии...",
                _ => "Restoring a mirrorlist backup...",
            });
            fs::remove_file("/etc/pacman.d/mirrorlist")?;
            fs::rename("/etc/pacman.d/mirrorlist.bak", "/etc/pacman.d/mirrorlist")?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    Ok(())
}
