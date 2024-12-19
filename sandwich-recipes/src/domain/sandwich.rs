use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sandwich {
    id: String,
    name: String,
    ingredients: Vec<String>,
}

impl Sandwich {
    pub fn new(id: String, name: String, ingredients: Vec<String>) -> Self {
        Self {
            id,
            name,
            ingredients,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SANDWICH_ID: &str = "sand-id";
    const SANDWICH_NAME: &str = "Hot dog";

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = vec!["Wurst".to_string(), "Ketchup".to_string()];

        let hot_dog = Sandwich::new(
            SANDWICH_ID.to_string(),
            SANDWICH_NAME.to_string(),
            ingredients.clone(),
            SandwichType::Meat,
        )
        .unwrap();

        assert_eq!(hot_dog.id().value().as_ref().unwrap(), SANDWICH_ID);
        assert_eq!(hot_dog.name.value(), SANDWICH_NAME);

        assert_eq!(ingredients.len(), hot_dog.ingredients.value().len());

        for (i, exp_ingr) in ingredients.iter().enumerate() {
            assert_eq!(exp_ingr, &hot_dog.ingredients.value()[i]);
        }
    }
}
