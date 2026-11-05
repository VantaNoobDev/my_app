use crate::options::Options;
use std::fs as fs;

pub struct Configuration {
    pub options: Options,
    pub sound_effects: bool,
    pub music: bool,
    pub notifications: bool,
}

pub fn default_configuration() -> Configuration { // Returns a default configuration with predefined settings
    Configuration {
        options: Options::default(),
        sound_effects: true,
        music: true,
        notifications: true,
    }
}

#[allow(dead_code)] // Suppresses warnings for unused functions that are defined but not called in the current context
pub fn load_configuration() -> Configuration { // Loads the configuration from a file, or returns the default configuration if the file does not exist
    fs::read_to_string("configuration.txt")
        .map(|contents| {
            let mut config = default_configuration();
            for line in contents.lines() {
                if let Some(value) = line.strip_prefix("Sound Effects: ") {
                    config.sound_effects = value == "On";
                } else if let Some(value) = line.strip_prefix("Music: ") {
                    config.music = value == "On";
                } else if let Some(value) = line.strip_prefix("Notifications: ") {
                    config.notifications = value == "On";
                } else if let Some(value) = line.strip_prefix("Difficulty: ") {
                    config.options.difficulty = value.to_string();
                } else if let Some(value) = line.strip_prefix("Graphics Quality: ") {
                    config.options.graphics_quality = value.to_string();
                } else if let Some(value) = line.strip_prefix("Sound Volume: ") {
                    if let Ok(volume) = value.parse::<u8>() {
                        config.options.sound_volume = volume;
                    }
                }
            }
            config
        })
        .unwrap_or_else(|_| default_configuration())
}

#[allow(dead_code)] // Suppresses warnings for unused functions that are defined but not called in the current context
pub fn save_configuration(config: &Configuration) { // Saves the current configuration to a file
    fs::write("configuration.txt", format!("Sound Effects: {}\nMusic: {}\nNotifications: {}\nDifficulty: {}\nGraphics Quality: {}\nSound Volume: {}",
        if config.sound_effects { "On" } else { "Off" },
        if config.music { "On" } else { "Off" },
        if config.notifications { "On" } else { "Off" },
        config.options.difficulty,
        config.options.graphics_quality,
        config.options.sound_volume))
        .expect("Failed to save configuration to file.");
}

#[allow(dead_code)] // Suppresses warnings for unused functions that are defined but not called in the current context
pub fn reset_configuration(config: &mut Configuration) { // Resets the configuration to default values
    *config = default_configuration();
}

#[allow(dead_code)] // Suppresses warnings for unused functions that are defined but not called in the current context
pub fn display_configuration(config: &Configuration) { // Displays the current configuration settings to the console
    println!("Configuration:");
    println!("Sound Effects: {}", if config.sound_effects { "On" } else { "Off" });
    println!("Music: {}", if config.music { "On" } else { "Off" });
    println!("Notifications: {}", if config.notifications { "On" } else { "Off" });
    config.options.display_options();
}