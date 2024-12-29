#[cfg(test)]
pub mod shared {
    use crate::domain::sandwich::{Sandwich, SandwichType};

    pub const SANDWICH_ID: &str = "sand-id";
    pub const SANDWICH_NAME: &str = "Hot dog";
    pub const SANDWICH_TYPE: SandwichType = SandwichType::Meat;
    pub const CHEESEBURGER_NAME: &str = "Cheeseburger";

    pub fn assert_on_sandwich(expected: Sandwich, actual: &Sandwich, assert_on_id: bool) {
        if assert_on_id {
            assert_eq!(
                actual.id().value().as_ref().unwrap(),
                expected.id().value().as_ref().unwrap()
            );
        }

        assert_eq!(actual.name().value(), expected.name().value());
    }

    pub fn assert_on_ingredients(
        expected_ingredients: &Vec<String>,
        actual_ingredients: &Vec<String>,
    ) {
        for (i, exp_ingr) in expected_ingredients.iter().enumerate() {
            assert_eq!(exp_ingr, &actual_ingredients[i]);
        }
    }
}
