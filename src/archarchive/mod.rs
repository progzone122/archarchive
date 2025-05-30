pub mod parser;
mod menu;
use crate::archarchive::menu::detect_language;
use anyhow::{Result, anyhow};

pub const ENDPOINT: &str = "https://archive.archlinux.org/repos";
pub struct ArchArchive {
    year: String,
    month: String,
    day: String
}
impl ArchArchive {
    pub fn init() -> ArchArchive {
        Self {
            year: String::new(),
            month: String::new(),
            day: String::new()
        }
    }
    fn get_link(&self) -> String {
        format!("{}/{}/{}/{}/$repo/os/$arch", ENDPOINT, self.year, self.month, self.day)
    }
    async fn select_date(&mut self) -> Result<()> {
        let lang = detect_language();
        
        let mut years = parser::parse_years().await?;
        years.reverse();
        if years.is_empty() {
            let msg = match lang.as_str() {
                "ru" => "Нет доступных годов",
                _ => "No available years found",
            };
            return Err(anyhow!(msg));
        }

        let prompt = match lang.as_str() {
            "ru" => "Выберите год: ",
            _ => "Select year: ",
        };
        self.year = menu::ask(years, prompt);
    
        let mut months = parser::parse_months(&self.year).await?;
        months.reverse();
        if months.is_empty() {
            let msg = match lang.as_str() {
                "ru" => "Нет доступных месяцев",
                _ => "No available months found",
            };
            return Err(anyhow!(msg));
        }

        let prompt = match lang.as_str() {
            "ru" => "Выберите месяц: ",
            _ => "Select month: ",
        };
        self.month = menu::ask(months, prompt);
    
        let mut days = parser::parse_days(&self.year, &self.month).await?;
        days.reverse();
        if days.is_empty() {
            let msg = match lang.as_str() {
                "ru" => "Нет доступных дней",
                _ => "No available days found",
            };
            return Err(anyhow!(msg));
        }

        let prompt = match lang.as_str() {
            "ru" => "Выберите день: ",
            _ => "Select day: ",
        };
        self.day = menu::ask(days, prompt);
    
        Ok(())
    }
    pub async fn menu_run(&mut self) -> Result<String> {
        self.select_date().await?;
        Ok(self.get_link())
    }
}
