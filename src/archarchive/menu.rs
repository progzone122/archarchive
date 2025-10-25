use inquire::error::InquireResult;
use inquire::{InquireError, Select};
use std::fmt::Display;
use std::{env, process};

fn create_prompt<T: Display + Clone>(items: Vec<T>, message: &str) -> InquireResult<T> {
    Select::new(message, items).prompt()
}

pub fn detect_language() -> String {
    let lang = env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "en".to_string()); // fallback

    lang.split('.')
        .next()
        .unwrap_or("en")
        .split('_')
        .next()
        .unwrap_or("en")
        .to_string()
}

pub fn ask<T: Display + Clone>(items: Vec<T>, message: &str) -> T {
    let lang = detect_language();
    loop {
        match create_prompt(items.clone(), message) {
            Ok(choice) => return choice,
            Err(err) => match err {
                InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                    match lang.as_str() {
                        "ru" => eprintln!("\nВыход..."),
                        _ => eprintln!("\nExiting..."),
                    }
                    process::exit(0);
                }
                _ => match lang.as_str() {
                    "ru" => eprintln!(
                        "Ошибка: {}. Попробуйте снова или нажмите Ctrl+C для выхода.",
                        err
                    ),
                    _ => eprintln!("Error: {}. Try again or press Ctrl+C to exit.", err),
                },
            },
        }
    }
}
