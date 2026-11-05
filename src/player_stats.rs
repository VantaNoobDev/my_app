use crate::coins::Coins;
use crate::inventory::Inventory;
use crate::options::{Difficulty, GraphicsQuality, Options};

#[cfg(test)]
use crate::options::{Language, SoundEffects};

use std::fs as fs;

pub struct PlayerStats {
    pub coins: Coins,
    pub inventory: Inventory,
    pub options: Options,
}

impl PlayerStats {
    pub fn reset_stats(&mut self) {
        self.coins.set_coins(0);
        self.inventory.clear_inventory();
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut contents = format!(
            "Coins: {}\n",
            self.coins.get_coins()
        );

        for item in self.inventory.get_items() {
            contents.push_str(&format!("Item: {}\n", item));
        }

        fs::write("player_stats.txt", contents)?;

        let options = format!(
            "Difficulty: {:?}\nGraphics Quality: {:?}\nSound Volume: {}",
            self.options.difficulty,
            match self.options.graphics_quality {
                GraphicsQuality::Low => "Low",
                GraphicsQuality::Medium => "Medium",
                GraphicsQuality::High => "High",
            },
            self.options.sound_volume
        );

        fs::write("options.txt", options)?;

        Ok(())
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let contents = fs::read_to_string("player_stats.txt")?;

        for line in contents.lines() {
            if let Some(value) = line.strip_prefix("Coins: ") {
                if let Ok(coins) = value.parse::<u32>() {
                    self.coins.set_coins(coins);
                }
                } else if let Some(item) = line.strip_prefix("Item: ") {
                    self.inventory.add_item(item.to_string());
                }
        }

        Ok(())
    }

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
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English }
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
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English } }
            .check_inventory();
        // This test will print the inventory contents to the console.
    }

    #[test]
    fn save_and_load_works() {
        assert!(fs::remove_file("player_stats.txt").is_ok() || fs::metadata("player_stats.txt").is_err());
        assert!(fs::remove_file("options.txt").is_ok() || fs::metadata("options.txt").is_err());

        let mut stats = PlayerStats {
            coins: Coins { amount: 5 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English } };
        
        stats.save()
            .expect("Failed to save player stats.");

        stats.coins.set_coins(0);
        stats.inventory.clear_inventory();

        stats.load()
            .expect("Failed to load player stats.");

        assert_eq!(stats.coins.get_coins(), 5);
        assert!(stats.inventory.has_item("Sword"));
        // Assuming save and load are implemented, this test would check if the stats are restored correctly
    }

    #[test]
    fn remove_item_not_found() {
        let mut stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English } };
        
        stats.inventory.remove_item("Shield"); // Should print "Item not found in inventory."
        assert!(stats.inventory.has_item("Sword")); // Sword should still be in the inventory
    }

    #[test]
    fn has_item_works() {
        let stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: vec!["Sword".to_string()] },
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English } };
        
        assert!(stats.inventory.has_item("Sword"));
        assert!(!stats.inventory.has_item("Shield"));
    }

    #[test]
    fn add_multiple_items() {
        let mut stats = PlayerStats {
            coins: Coins { amount: 0 },
            inventory: Inventory { items: Vec::new() },
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English } };
        
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
            options: Options { difficulty: Difficulty::Normal, graphics_quality: GraphicsQuality::Medium, sound_volume: 3, sound_effects: SoundEffects::On, language: Language::English } };
        
        stats.inventory.remove_item("Sword");
        
        assert!(!stats.inventory.has_item("Sword"));
        assert!(stats.inventory.has_item("Shield"));
    }
}