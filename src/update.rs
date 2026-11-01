use self_update::cargo_crate_version;
use std::env;
use std::process::{Command, exit};
const REPO_OWNER: &str = "fufliik";
const REPO_NAME: &str = "Glava_Launcher";
const BIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check() -> Result<bool, Box<dyn std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build()?
        .fetch()?
        .with_current_version(VERSION);

    if releases.is_update_available()? {
        if let Some(latest) = releases.latest() {
            println!("Найдено новое обновление: {} -> {}", VERSION, latest.version());
        }
        //run()?;
        return Ok(true);

    }
    Ok(false)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(true)
        .target("x86_64-pc-windows-msvc")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Обновлено до {}", status.version());
    restart_process_app();
    Ok(())
}

pub fn restart_process_app() {
    if let Ok(current_exe) = env::current_exe() {
        let args: Vec<String> = env::args().skip(1).collect();
        let mut cmd = Command::new(current_exe);
        cmd.args(&args);
        if cmd.spawn().is_ok() {
            exit(0);
        } else {
            eprintln!("Не удалось запустить новую копию приложения.");
        }
    }
}


