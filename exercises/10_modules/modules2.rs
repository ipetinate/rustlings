// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

use crate::delicious_snacks::fruits;
use crate::delicious_snacks::veggies;

fn main() {
    println!(
        "favorite snacks: {} and {}",
        veggies::CUCUMBER,
        fruits::PEAR,
    );
}
