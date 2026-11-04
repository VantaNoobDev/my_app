pub struct ItemInfo {
    pub name: &'static str,
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