pub struct Inventory {
    pub items: Vec<String>,
}

impl Inventory {
    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
            true
        } else {
            println!("Item not found in inventory.");
            false
        }
    }

    pub fn get_items(&self) -> &[String] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
    }

    pub fn clear_inventory(&mut self) {
        self.items.clear();
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.items.iter().any(|x| x == item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_add_remove_works() {
        let mut inventory = Inventory { items: Vec::new() };

        inventory.add_item("Sword".to_string());
        assert!(inventory.has_item("Sword"));

        inventory.remove_item("Sword");
        assert!(!inventory.has_item("Sword"));
    }

    #[test]
    fn inventory_clear_works() {
        let mut inventory = Inventory { items: Vec::new() };

        inventory.add_item("Sword".to_string());
        inventory.clear_inventory();

        assert!(!inventory.has_item("Sword"));
    }

    #[test]
    fn inventory_set_get_works() {
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