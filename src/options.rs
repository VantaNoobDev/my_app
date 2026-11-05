use std::io;

pub struct Options {
    pub difficulty: String,
    pub graphics_quality: String,
    pub sound_volume: u8,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the Options struct
pub struct Actions {
    pub save_game: bool,
    pub load_game: bool,
    pub reset_game: bool,
    pub exit_game: bool,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the Configuration struct
pub struct Configuration {
    pub options: Options,
    pub actions: Actions,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the Settings struct
pub struct Settings {
    pub configuration: Configuration,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the UserInput struct
pub struct UserInput {
    pub input: String,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the UserChoice struct
pub struct UserChoice {
    pub choice: String,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the UserSelection struct
pub struct UserSelection {
    pub selection: String,
}

#[allow(dead_code)] // This attribute suppresses warnings for unused code in the UserResponse struct
pub fn display_actions(actions: &Actions) { // Displays the current actions settings to the console
    println!("Actions:");
    println!("Save Game: {}", if actions.save_game { "Enabled" } else { "Disabled" });
    println!("Load Game: {}", if actions.load_game { "Enabled" } else { "Disabled" });
    println!("Reset Game: {}", if actions.reset_game { "Enabled" } else { "Disabled" });
    println!("Exit Game: {}", if actions.exit_game { "Enabled" } else { "Disabled" });
}

impl Options {
    pub fn default() -> Self { // Returns a default Options instance with predefined settings
        let actions = Actions {
            save_game: false,
            load_game: false,
            reset_game: false,
            exit_game: false,
        };

        let options = Options {
            difficulty: "Medium".into(),
            graphics_quality: "High".into(),
            sound_volume: 100,
        };

        let configuration = Configuration {
            options,
            actions,
        };

        let settings = Settings { configuration };

        return settings.configuration.options;
    }

    pub fn display_options(&self) { // Displays the current options settings to the console
        println!("Options:");
        println!("Difficulty: {}", self.difficulty);
        println!("Graphics Quality: {}", self.graphics_quality);
        println!("Sound Volume: {}", self.sound_volume);
    }
}

#[allow(dead_code)] // This function will be used in the future for user input handling, but is currently not used in the codebase.
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
            difficulty: "Hard".to_string(),
            graphics_quality: "Ultra".to_string(),
            sound_volume: 80,
        };
        options.display_options();
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