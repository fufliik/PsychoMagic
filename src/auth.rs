use lyceris::minecraft::config::Memory;
use crate::dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub username: String,

}

impl User {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(dir::user()?, json)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(dir::user()?)?;
        if json.trim().is_empty() {
            return Ok(Self::default()); // пустой файл: первый запуск
        }
        Ok(serde_json::from_str(&json)?)
    }
}