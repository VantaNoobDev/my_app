use crate::player_stats::PlayerStats;
use crate::item_info as info;

pub fn buy_item(player: &mut PlayerStats, item_name: &str) -> bool {
    let Some(item) = info::get_item(item_name) else {
        println!("Unknown item: {item_name}");
        return false;
    };

    if !player.coins.remove_coins(item.buy_price) {
        println!("Not enough coins to buy {}.", item.name);
        return false;
    }

    player.inventory.add_item(item.name.to_string());
    println!("Bought {}!", item.name);
    true
}

pub fn sell_item(player: &mut PlayerStats, item_name: &str) -> bool {
    let Some(item) = info::get_item(item_name) else {
        println!("Unknown item: {item_name}");
        return false;
    };

    if !player.inventory.remove_item(item.name) {
        println!("You don't have a {} to sell.", item.name);
        return false;
    }

    player.coins.add_coins(item.sell_price);
    println!("Sold {}!", item.name);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coins::Coins;
    use crate::inventory::Inventory;

    fn make_player(coins: u32) -> PlayerStats {
        PlayerStats {
            coins: Coins { amount: coins },
            inventory: Inventory { items: Vec::new() },
        }
    }

    #[test]
    fn buy_item_works() {
        let mut player = make_player(10);

        assert!(buy_item(&mut player, "Potion"));
        assert_eq!(player.coins.get_coins(), 6);
        assert!(player.inventory.has_item("Potion"));
    }

    #[test]
    fn cannot_buy_without_enough_coins() {
        let mut player = make_player(3);

        assert!(!buy_item(&mut player, "Potion"));
        assert_eq!(player.coins.get_coins(), 3);
        assert!(!player.inventory.has_item("Potion"));
    }

    #[test]
    fn sell_item_works() {
        let mut player = make_player(10);
        player.inventory.add_item("Potion".to_string());

        assert!(sell_item(&mut player, "Potion"));
        assert_eq!(player.coins.get_coins(), 12);
        assert!(!player.inventory.has_item("Potion"));
    }
}