mod coins;
mod inventory;
mod player_stats;
mod shop;
mod item_info;
mod options;
mod configuration;

use coins::Coins;
use inventory::Inventory;
use player_stats::PlayerStats;
use options::Options;
use configuration::Configuration;

fn create() -> PlayerStats { // Creates a new PlayerStats instance with default values
    PlayerStats {
        coins: Coins { amount: 0 },
        inventory: Inventory { items: Vec::new() },
        options: Options::default(),
    }
}

fn main() { // Entry point of the application, where the main logic is executed
    let mut player_stats = create();

    //player_stats.coins.add_coins(10);
    
    //println!("Player has {} coins.", player_stats.coins.get_coins());

    //shop::buy_item(&mut player_stats, "Potion");

    //println!("Player has {} coins.", player_stats.coins.get_coins());

    //shop::sell_item(&mut player_stats, "Potion");

    //println!("Coins left: {}", player_stats.coins.get_coins());

    let configuration = Configuration 
    { 
        options: Options::default(),
        sound_effects: true,
        music: true,
        notifications: true
    };

    configuration.options.display_options();
    player_stats.load();
}