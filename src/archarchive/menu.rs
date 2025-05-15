use std::fmt::Display;
use inquire::error::InquireResult;
use inquire::Select;

fn create_prompt<T: Display + Clone>(items: Vec<T>, message: &str) -> InquireResult<T> {
    Select::new(message, items)
        .prompt()
}

pub fn ask<T: Display + Clone>(items: Vec<T>, message: &str) -> T {
    loop {
        match create_prompt(items.clone(), message) {
            Ok(choice) => return choice,
            Err(err) => {
                eprintln!("Ошибка: {}. Попробуйте снова или Ctrl+C для выхода.", err);
            },
        }
    }
}