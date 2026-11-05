pub struct Coins {
    pub amount: u32,
}

impl Coins {
    pub fn add_coins(&mut self, amount: u32) { // Adds a specified amount of coins to the current amount by incrementing the amount field
        self.amount += amount;
    }

    pub fn remove_coins(&mut self, amount: u32) -> bool { // Removes a specified amount of coins from the current amount if there are enough coins available, returning true if successful and false if there are insufficient funds
        if self.amount < amount {
            return false;
        }

        self.amount -= amount;
        true
    }

    pub fn set_coins(&mut self, amount: u32) { // Sets the current amount of coins to a specified value by assigning the amount field
        self.amount = amount;
    }

    pub fn get_coins(&self) -> u32 { // Returns the current amount of coins by accessing the amount field
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() { // Tests the add_coins method of the Coins struct to ensure that coins can be added correctly
        let mut coins = Coins { amount: 0 };
        coins.add_coins(5);
        assert_eq!(coins.get_coins(), 5);
    }

    #[test]
    fn remove_works() { // Tests the remove_coins method of the Coins struct to ensure that coins can be removed correctly and that the method returns true when successful
        let mut coins = Coins { amount: 5 };

        assert!(coins.remove_coins(3));
        assert_eq!(coins.get_coins(), 2);
    }

    #[test]
    fn set_coins_works() { // Tests the set_coins method of the Coins struct to ensure that the amount of coins can be set correctly
        let mut coins = Coins { amount: 3 };
        coins.set_coins(4);
        assert_eq!(coins.get_coins(), 4);
    }

    #[test]
    fn get_coins_works() { // Tests the get_coins method of the Coins struct to ensure that it returns the correct amount of coins
        let coins = Coins { amount: 5 };
        assert_eq!(coins.get_coins(), 5);
    }

    #[test]
    fn remove_coins_insufficient_funds() { // Tests the remove_coins method of the Coins struct to ensure that it returns false when attempting to remove more coins than are available, and that the amount of coins remains unchanged
        let mut coins = Coins { amount: 5 };

        assert!(!coins.remove_coins(10));
        assert_eq!(coins.get_coins(), 5);
    }
}