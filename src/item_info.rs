pub struct ItemInfo {
    pub name: &'static str,
    pub buy_price: u32,
    pub sell_price: u32,
}

pub struct Item {
    pub name: String,
    pub buy_price: u32,
    pub sell_price: u32,
}

pub fn get_item(name: &str) -> Option<ItemInfo> {
    match name {
        "Potion" => Some(ItemInfo {
            name: "Potion",
            buy_price: 4,
            sell_price: 2,
        }),
        "Sword" => Some(ItemInfo {
            name: "Sword",
            buy_price: 10,
            sell_price: 5,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_item() { // Tests the get_item function for a known item
        let potion_info = get_item("Potion").unwrap();
        assert_eq!(potion_info.name, "Potion");
        assert_eq!(potion_info.buy_price, 4);
        assert_eq!(potion_info.sell_price, 2);
    }

    #[test]
    fn test_get_unknown_item() { // Tests the get_item function for an unknown item
        let unknown_info = get_item("Unknown");
        assert!(unknown_info.is_none());
    }
}