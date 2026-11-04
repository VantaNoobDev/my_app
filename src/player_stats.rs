use crate::coins::Coins;
use crate::inventory::Inventory;

pub struct PlayerStats {
    pub coins: Coins,
    pub inventory: Inventory,
}

impl PlayerStats {
    pub fn reset_stats(&mut self) {
        self.coins.set_coins(0);
        self.inventory.clear_inventory();
    }

    fn save(&self) {
        // Save player stats to a file or database, later.
    }

    fn load(&mut self) {
        // Load player stats from a file or database, later.
    }

    fn check_inventory(&self) {
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
        let mut stats = PlayerStats { coins: Coins { amount: 5 }, inventory: Inventory { items: vec!["Sword".to_string()] } };
        stats.reset_stats();
        assert_eq!(stats.coins.get_coins(), 0);
        assert!(!stats.inventory.has_item("Sword"));
    }

    #[test]
    fn check_inventory_prints_items() {
        PlayerStats { coins: Coins { amount: 0 }, inventory: Inventory { items: vec!["Sword".to_string(), "Shield".to_string()] } }.check_inventory();
        // This test will print the inventory contents to the console.
    }

    #[test]
    fn save_and_load_works() {
        let mut stats = PlayerStats { coins: Coins { amount: 5 }, inventory: Inventory { items: vec!["Sword".to_string()] } };
        stats.save();
        stats.coins.set_coins(0);
        stats.inventory.clear_inventory();
        stats.load();
        // Assuming save and load are implemented, this test would check if the stats are restored correctly
    }

    #[test]
    fn remove_item_not_found() {
        let mut stats = PlayerStats { coins: Coins { amount: 0 }, inventory: Inventory { items: vec!["Sword".to_string()] } };
        stats.inventory.remove_item("Shield"); // Should print "Item not found in inventory."
        assert!(stats.inventory.has_item("Sword")); // Sword should still be in the inventory
    }

    #[test]
    fn has_item_works() {
        let stats = PlayerStats { coins: Coins { amount: 0 }, inventory: Inventory { items: vec!["Sword".to_string()] } };
        assert!(stats.inventory.has_item("Sword"));
        assert!(!stats.inventory.has_item("Shield"));
    }

    #[test]
    fn add_multiple_items() {
        let mut stats = PlayerStats { coins: Coins { amount: 0 }, inventory: Inventory { items: Vec::new() } };
        stats.inventory.add_item("Sword".to_string());
        stats.inventory.add_item("Shield".to_string());
        assert!(stats.inventory.has_item("Sword"));
        assert!(stats.inventory.has_item("Shield"));
    }

    #[test]
    fn remove_multiple_items() {
        let mut stats = PlayerStats { coins: Coins { amount: 0 }, inventory: Inventory { items: vec!["Sword".to_string(), "Shield".to_string()] } };
        stats.inventory.remove_item("Sword");
        assert!(!stats.inventory.has_item("Sword"));
        assert!(stats.inventory.has_item("Shield"));
    }
}