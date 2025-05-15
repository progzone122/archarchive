mod archarchive;

use std::fs;
use std::io::Write;
use anyhow::{Result, anyhow};
use tokio::process::Command;
use archarchive::{ArchArchive, parser};

#[tokio::main]
async fn main() -> Result<()> {

    let mut aa = ArchArchive::init();
    match aa.menu_run().await {
        Ok(url) => {
            println!("URL: {}", url);

            println!("Creating a mirrorlist backup...");
            fs::rename("/etc/pacman.d/mirrorlist", "/etc/pacman.d/mirrorlist.bak")?;

            println!("Writing an archarchive mirror to mirrorlist...");
            let mut file = fs::File::create("/etc/pacman.d/mirrorlist")?;
            file.write_all(format!("Server = {url}").as_bytes())?;

            Command::new("sudo")
                .args(["pacman", "-Syyuu"])
                .status()
                .await?;

            println!("Restoring a mirrorlist backup...");
            fs::remove_file("/etc/pacman.d/mirrorlist")?;
            fs::rename("/etc/pacman.d/mirrorlist.bak", "/etc/pacman.d/mirrorlist")?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    Ok(())
}
