pub mod parser;
mod menu;

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
    let mut years = parser::parse_years().await?;
    years.reverse();
    self.year = menu::ask(years, "Select year: ");

    let mut months = parser::parse_months(&self.year).await?;
    months.reverse();
    self.month = menu::ask(months, "Select month: ");

    let mut days = parser::parse_days(&self.year, &self.month).await?;
    days.reverse();
    self.day = menu::ask(days, "Select day: ");

    Ok(())
    }
    pub async fn menu_run(&mut self) -> Result<String> {
        self.select_date().await?;
        Ok(self.get_link())
    }
}
