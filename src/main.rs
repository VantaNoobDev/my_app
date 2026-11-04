mod coins;
mod inventory;
mod player_stats;
mod shop;
mod item_info;

use coins::Coins;
use inventory::Inventory;
use player_stats::PlayerStats;

fn create() -> PlayerStats {
    PlayerStats {
        coins: Coins { amount: 0 },
        inventory: Inventory { items: Vec::new() },
    }
}

fn main() {
    let mut player_stats = create();

    player_stats.coins.add_coins(10);
    
    println!("Player has {} coins.", player_stats.coins.get_coins());

    shop::buy_item(&mut player_stats, "Potion");

    println!("Player has {} coins.", player_stats.coins.get_coins());

    shop::sell_item(&mut player_stats, "Potion");

    println!("Coins left: {}", player_stats.coins.get_coins());
}