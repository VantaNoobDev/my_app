pub struct Coins {
    pub amount: u32,
}

impl Coins {
    pub fn add_coins(&mut self, amount: u32) {
        self.amount += amount;
    }

    pub fn remove_coins(&mut self, amount: u32) -> bool {
        if self.amount < amount {
            return false;
        }

        self.amount -= amount;
        true
    }

    pub fn set_coins(&mut self, amount: u32) {
        self.amount = amount;
    }

    pub fn get_coins(&self) -> u32 {
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let mut coins = Coins { amount: 0 };
        coins.add_coins(5);
        assert_eq!(coins.get_coins(), 5);
    }

    #[test]
    fn remove_works() {
        let mut coins = Coins { amount: 5 };

        assert!(coins.remove_coins(3));
        assert_eq!(coins.get_coins(), 2);
    }

    #[test]
    fn set_coins_works() {
        let mut coins = Coins { amount: 3 };
        coins.set_coins(4);
        assert_eq!(coins.get_coins(), 4);
    }

    #[test]
    fn get_coins_works() {
        let coins = Coins { amount: 5 };
        assert_eq!(coins.get_coins(), 5);
    }

    #[test]
    fn remove_coins_insufficient_funds() {
        let mut coins = Coins { amount: 5 };

        assert!(!coins.remove_coins(10));
        assert_eq!(coins.get_coins(), 5);
    }
}