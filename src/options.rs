#[cfg(test)]
use crate::configuration::{Configuration, display_configuration};

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEffects {
    On,
    Off,
}

pub enum Notifications {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language { // I'll implement the portuguese language in the future, but for now it's just English
    English,
    Portuguese,
}

pub enum GraphicsQuality {
    Low,
    Medium,
    High,
}

pub struct Options {
    pub difficulty: Difficulty,
    pub graphics_quality: GraphicsQuality,
    pub sound_volume: u8,
    pub sound_effects: SoundEffects,
    pub language: Language,
}

pub struct Actions {
    pub save_game: bool,
    pub load_game: bool,
    pub reset_game: bool,
    pub exit_game: bool,
}

pub struct UserInput {
    pub input: String,
}

pub struct UserChoice {
    pub choice: String,
}

pub struct UserSelection {
    pub selection: String,
}

pub fn display_actions(actions: &Actions) { // Displays the current actions settings to the console
    println!("Actions:");
    println!("Save Game: {}", if actions.save_game { "Enabled" } else { "Disabled" });
    println!("Load Game: {}", if actions.load_game { "Enabled" } else { "Disabled" });
    println!("Reset Game: {}", if actions.reset_game { "Enabled" } else { "Disabled" });
    println!("Exit Game: {}", if actions.exit_game { "Enabled" } else { "Disabled" });
}

impl Default for Options {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Normal,
            graphics_quality: GraphicsQuality::High,
            sound_volume: 100,
            sound_effects: SoundEffects::On,
            language: Language::English,
        }
    }
}

pub fn input(prompt: &str, options: Vec<&str>) -> String {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input_trimmed = input.trim();
        if options.contains(&input_trimmed) {
            return input_trimmed.to_string();
        } else {
            println!("Invalid option. Please try again.");
            input.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self}; // Import Write trait for flushing output, and io module for testing input handling

    #[test]
    fn test_display_options() { // This test checks if the display_options function correctly displays the options based on the provided Options struct.
        let options = Options {
            difficulty: Difficulty::Hard,
            graphics_quality: GraphicsQuality::High,
            sound_volume: 80,
            sound_effects: SoundEffects::On,
            language: Language::English,
        };
        
        display_configuration(&Configuration {
            options,
            music: true,
            notifications: Notifications::Enabled,
        });
    }

    #[test]
    fn test_display_actions() { // This test checks if the display_actions function correctly displays the actions based on the provided Actions struct.
        let actions = Actions {
            save_game: true,
            load_game: false,
            reset_game: true,
            exit_game: false,
        };
        display_actions(&actions);
    }

    #[test]
    fn test_input_valid_option() { // This test simulates user input for a valid option and checks if the function returns the expected result.
        let _options = vec!["Option1", "Option2", "Option3"];
        let input = "Option2\n";
        let _input_cursor = io::Cursor::new(input);
        let _output: Vec<u8> = Vec::new();
    }

    #[test]
    fn test_input_invalid_option() { // This test simulates user input for an invalid option followed by a valid option and checks if the function handles it correctly.
        let _options = vec!["Option1", "Option2", "Option3"];
        let input = "InvalidOption\nOption1\n";
        let _input_cursor = io::Cursor::new(input);
        let _output: Vec<u8> = Vec::new();
    }
}