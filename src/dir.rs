use std::path::PathBuf;
use std::io;
use std::fs;
pub fn launcher() -> io::Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Local data directory not found",
        ))?;
    let launcher_dir = data_dir.join(".PsychoMagic");
    fs::create_dir_all(&launcher_dir)?;

    Ok(launcher_dir)
}

pub fn user() -> io::Result<PathBuf> {
    let cfg = launcher()?.join("user.json");
    if !cfg.exists() {
        fs::File::create(&cfg)?;
    }
    Ok(cfg)
}

pub fn settings() -> io::Result<PathBuf> {
    let cfg = launcher()?.join("settings.json");
    if !cfg.exists() {
        fs::File::create(&cfg)?;
    }
    Ok(cfg)
}
