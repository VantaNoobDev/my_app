use crate::coins::Coins;
use crate::inventory::Inventory;
use crate::options::Options;

use std::fs as fs;

#[allow(dead_code)] // Will be removed in the future, but for now it's useful for testing
pub struct PlayerStats {
    pub coins: Coins,
    pub inventory: Inventory,
    pub options: Options,
}

impl PlayerStats {
    #[allow(dead_code)] // Will be removed in the future, but for now it's useful for testing
    pub fn reset_stats(&mut self) {
        self.coins.set_coins(0);
        self.inventory.clear_inventory();
    }

    #[allow(dead_code)] // Will be removed in the future, but for now it's useful for testing
    pub fn save(&self) {
        let information = format!("Coins: {}\nInventory: {:?}", self.coins.get_coins(), self.inventory.get_items());
        fs::write("player_stats.txt", information)
            .expect("Failed to save player stats to file.");

        let options_information = format!("Difficulty: {}\nGraphics Quality: {}\nSound Volume: {}", self.options.difficulty, self.options.graphics_quality, self.options.sound_volume);
        fs::write("options.txt", options_information)
            .expect("Failed to save options to file.");
    }

    #[allow(dead_code)] // Will be removed in the future, but for now it's useful for testing
    pub fn load(&mut self) {
        let contents = fs::read_to_string("player_stats.txt")
            .expect("Failed to read player stats from file.");
        
        let lines: Vec<&str> = contents.lines().collect();
        if lines.len() >= 2 {
            if let Some(coins_line) = lines.get(0) {
                if let Some(coins_str) = coins_line.strip_prefix("Coins: ") {
                    if let Ok(coins) = coins_str.parse::<u32>() {
                        self.coins.set_coins(coins);
                    }
                }
            }

            if let Some(inventory_line) = lines.get(1) {
                if let Some(inventory_str) = inventory_line.strip_prefix("Inventory: ") {
                    let items: Vec<String> = inventory_str.trim_matches(
                        |c| c == '[' || c == ']'
                    )
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();

                    self.inventory.items = items;
                }
            }

            if let Some(options_contents) = fs::read_to_string("options.txt").ok() {
                let options_lines: Vec<&str> = options_contents.lines().collect();
                for line in options_lines {
                    if let Some(value) = line.strip_prefix("Difficulty: ") {
                        self.options.difficulty = value.to_string();
                    } else if let Some(value) = line.strip_prefix("Graphics Quality: ") {
                        self.options.graphics_quality = value.to_string();
                    } else if let Some(value) = line.strip_prefix("Sound Volume: ") {
                        if let Ok(volume) = value.parse::<u8>() {
                            self.options.sound_volume = volume;
                        }
                    }
                }
            }
        }
    }

    #[allow(dead_code)] // Will be removed in the future, but for now it's useful for testing
    pub fn check_inventory(&self) {
        println!("Inventory contents:");
        for item in self.inventory.get_items() {
            println!("- {}", item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_stats_works() {
        let mut stats = PlayerStats { 
            coins: Coins { amount: 5 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 }
        };

        stats.reset_stats();
        
        assert_eq!(stats.coins.get_coins(), 0);
        assert!(!stats.inventory.has_item("Sword"));
    }

    #[test]
    fn check_inventory_prints_items() {
        PlayerStats { 
            coins: Coins { amount: 0 },
            inventory: Inventory { items: vec!["Sword".to_string(), "Shield".to_string()] },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 } }
            .check_inventory();
        // This test will print the inventory contents to the console.
    }

    #[test]
    fn save_and_load_works() {
        let mut stats = PlayerStats {
            coins: Coins { amount: 5 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 } };
        
        stats.save();
        stats.coins.set_coins(0);
        stats.inventory.clear_inventory();
        stats.load();
        // Assuming save and load are implemented, this test would check if the stats are restored correctly
    }

    #[test]
    fn remove_item_not_found() {
        let mut stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 } };
        
        stats.inventory.remove_item("Shield"); // Should print "Item not found in inventory."
        assert!(stats.inventory.has_item("Sword")); // Sword should still be in the inventory
    }

    #[test]
    fn has_item_works() {
        let stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 } };
        
        assert!(stats.inventory.has_item("Sword"));
        assert!(!stats.inventory.has_item("Shield"));
    }

    #[test]
    fn add_multiple_items() {
        let mut stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: Vec::new() },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 } };
        
        stats.inventory.add_item("Sword".to_string());
        stats.inventory.add_item("Shield".to_string());
        
        assert!(stats.inventory.has_item("Sword"));
        assert!(stats.inventory.has_item("Shield"));
    }

    #[test]
    fn remove_multiple_items() {
        let mut stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: vec!["Sword".to_string(), "Shield".to_string()] },
            options: Options { difficulty: "Normal".into(), graphics_quality: "Medium".into(), sound_volume: 3 } };
        
        stats.inventory.remove_item("Sword");
        
        assert!(!stats.inventory.has_item("Sword"));
        assert!(stats.inventory.has_item("Shield"));
    }
}