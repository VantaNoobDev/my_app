use crate::options::{Difficulty, GraphicsQuality, Notifications, Options, SoundEffects};
use std::fs as fs;

pub struct Configuration {
    pub options: Options,
    pub music: bool,
    pub notifications: Notifications,
}

pub fn default_configuration() -> Configuration { // Returns a default configuration with predefined settings
    Configuration {
        options: Options::default(),
        music: true,
        notifications: Notifications::Enabled,
    }
}

pub fn load_configuration() -> Configuration { // Loads the configuration from a file, or returns the default configuration if the file does not exist
    fs::read_to_string("configuration.txt")
        .map(|contents| {
            let mut config = default_configuration();
            for line in contents.lines() {
                if let Some(value) = line.strip_prefix("Sound Effects: ") {
                    config.options.sound_effects = match value {
                        "On" => SoundEffects::On,
                        "Off" => SoundEffects::Off,
                        _ => SoundEffects::On,
                    };
                } else if let Some(value) = line.strip_prefix("Music: ") {
                    config.music = value == "On";
                } else if let Some(value) = line.strip_prefix("Notifications: ") {
                    config.notifications = match value {
                        "On" => Notifications::Enabled,
                        "Off" => Notifications::Disabled,
                        _ => Notifications::Enabled,
                    };
                } else if let Some(value) = line.strip_prefix("Difficulty: ") {
                    config.options.difficulty = match value {
                        "Easy" => Difficulty::Easy,
                        "Normal" => Difficulty::Normal,
                        "Hard" => Difficulty::Hard,
                        _ => Difficulty::Normal,
                        
                    };
                } else if let Some(value) = line.strip_prefix("Graphics Quality: ") {
                    config.options.graphics_quality = match value {
                        "Low" => GraphicsQuality::Low,
                        "Medium" => GraphicsQuality::Medium,
                        "High" => GraphicsQuality::High,
                        _ => GraphicsQuality::Medium,
                    };
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


pub fn save_configuration(config: &Configuration) { // Saves the current configuration to a file
    fs::write("configuration.txt", format!("Sound Effects: {}\nMusic: {}\nNotifications: {}\nDifficulty: {}\nGraphics Quality: {}\nSound Volume: {}",
        match config.options.sound_effects {
            SoundEffects::On => "On",
            SoundEffects::Off => "Off",
        },
        if config.music { "On" } else { "Off" },
        match config.notifications {
            Notifications::Enabled => "On",
            Notifications::Disabled => "Off",
        },
        match config.options.difficulty {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        },
        match config.options.graphics_quality {
            GraphicsQuality::Low => "Low",
            GraphicsQuality::Medium => "Medium",
            GraphicsQuality::High => "High",
        },
        config.options.sound_volume))
        .expect("Failed to save configuration to file.");
}

pub fn reset_configuration(config: &mut Configuration) { // Resets the configuration to default values
    *config = default_configuration();
}

pub fn display_configuration(config: &Configuration) { // Displays the current configuration settings to the console
    println!("Configuration:");
    println!("Sound Effects: {}", match config.options.sound_effects {
        SoundEffects::On => "On",
        SoundEffects::Off => "Off",
    });
    println!("Music: {}", match config.music {
        true => "On",
        false => "Off",
    });
    println!("Notifications: {}", match config.notifications {
        Notifications::Enabled => "On",
        Notifications::Disabled => "Off",
    });
    println!("Difficulty: {}", match config.options.difficulty {
        Difficulty::Easy => "Easy",
        Difficulty::Normal => "Normal",
        Difficulty::Hard => "Hard",
    });
    println!("Graphics Quality: {}", match config.options.graphics_quality {
        GraphicsQuality::Low => "Low",
        GraphicsQuality::Medium => "Medium",
        GraphicsQuality::High => "High",
    });
    println!("Sound Volume: {}", config.options.sound_volume);
}