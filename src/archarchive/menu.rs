use std::fmt::Display;
use inquire::error::InquireResult;
use inquire::Select;
use std::env;

fn create_prompt<T: Display + Clone>(items: Vec<T>, message: &str) -> InquireResult<T> {
    Select::new(message, items)
        .prompt()
}

fn detect_language() -> String {
    let lang = env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "en".to_string()); // fallback

    lang.split('.').next().unwrap_or("en").split('_').next().unwrap_or("en").to_string()
}

pub fn ask<T: Display + Clone>(items: Vec<T>, message: &str) -> T {
    let lang = detect_language();
    loop {
        match create_prompt(items.clone(), message) {
            Ok(choice) => return choice,
            Err(err) => {
                match lang.as_str() {
                    "ru" => eprintln!("Ошибка: {}. Попробуйте снова или нажмите Ctrl+C для выхода.", err),
                    _     => eprintln!("Error: {}. Try again or use Ctrl+C to exit.", err),
                }
            },
        }
    }
}
