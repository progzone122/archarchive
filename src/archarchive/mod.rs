pub mod parser;
mod menu;

use anyhow::{Result, anyhow};
use scraper::{Html, Selector};

pub const ENDPOINT: &str = "https://archive.archlinux.org/repos";
pub struct ArchArchive {
    year: String,
    month: String,
    day: String
}
impl ArchArchive {
    pub fn init() -> ArchArchive {
        Self {
            year: "2025".to_string(),
            month: "05".to_string(),
            day: "15".to_string()
        }
    }
    fn get_link(&self) -> String {
        format!("{}/{}/{}/{}/$repo/os/$arch", ENDPOINT, self.year, self.month, self.day)
    }

    pub async fn menu_run(&mut self) -> Result<String> {
        let mut years: Vec<String> = parser::parse_years().await?;
        years.reverse();
        self.year = menu::ask(years, "Select year: ");

        let mut months: Vec<String> = parser::parse_months(&self.year).await?;
        months.reverse();
        self.month = menu::ask(months, "Select month: ");

        let mut days: Vec<String> = parser::parse_days(&self.year, &self.month).await?;
        days.reverse();
        self.day = menu::ask(days, "Select day: ");

        Ok(self.get_link().to_string())
    }
}