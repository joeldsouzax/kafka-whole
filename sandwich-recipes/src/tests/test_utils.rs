#[cfg(test)]
pub mod shared {
    use crate::domain::sandwich::{Sandwich, SandwichType};

    pub const SANDWICH_ID: &str = "sand-id";
    pub const SANDWICH_NAME: &str = "Hot dog";
    pub const SANDWICH_TYPE: SandwichType = SandwichType::Meat;
    pub const CHEESEBURGER_NAME: &str = "Cheeseburger";
}
