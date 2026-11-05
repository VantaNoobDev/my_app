pub struct Inventory {
    pub items: Vec<String>,
}

impl Inventory {
    pub fn add_item(&mut self, item: String) { // Adds an item to the inventory by pushing it onto the items vector
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool { // Removes an item from the inventory if it exists, returning true if successful and false if the item was not found
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
            true
        } else {
            println!("Item not found in inventory.");
            false
        }
    }

    pub fn get_items(&self) -> &[String] { // Returns a slice of the items vector, allowing read-only access to the inventory items
        &self.items
    }

    #[allow(dead_code)] // Suppresses warnings for unused functions that are defined but not called in the current context
    pub fn set_items(&mut self, items: Vec<String>) { // Sets the items vector to a new vector of items, replacing the current inventory
        self.items = items;
    }

    pub fn clear_inventory(&mut self) { // Clears all items from the inventory by calling the clear method on the items vector
        self.items.clear();
    }

    #[allow(dead_code)] // Suppresses warnings for unused functions that are defined but not called in the current context
    pub fn has_item(&self, item: &str) -> bool { // Checks if the inventory contains a specific item by using the any method on the items vector, returning true if the item is found and false otherwise
        self.items.iter().any(|x| x == item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_add_remove_works() { // Tests the add_item and remove_item methods of the Inventory struct to ensure that items can be added and removed correctly
        let mut inventory = Inventory { items: Vec::new() };

        inventory.add_item("Sword".to_string());
        assert!(inventory.has_item("Sword"));

        inventory.remove_item("Sword");
        assert!(!inventory.has_item("Sword"));
    }

    #[test]
    fn inventory_clear_works() { // Tests the clear_inventory method of the Inventory struct to ensure that all items can be removed from the inventory
        let mut inventory = Inventory { items: Vec::new() };

        inventory.add_item("Sword".to_string());
        inventory.clear_inventory();

        assert!(!inventory.has_item("Sword"));
    }

    #[test]
    fn inventory_set_get_works() { // Tests the set_items and get_items methods of the Inventory struct to ensure that items can be set and retrieved correctly
        let mut inventory = Inventory { items: Vec::new() };

        inventory.set_items(vec![
            "Sword".to_string(),
            "Shield".to_string(),
        ]);

        let items = inventory.get_items();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&"Sword".to_string()));
        assert!(items.contains(&"Shield".to_string()));
    }
}