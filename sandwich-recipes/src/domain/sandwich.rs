use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sandwich {
    id: String,
    name: String,
    ingredients: Vec<String>,
}

impl Sandwich {
    pub fn new(id: String, name: String, ingredients: Vec<String>) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Any sandwich must have a name".to_string());
        }

        if ingredients.is_empty() {
            return Err("Any sandwich must have at least an ingredient".to_string());
        }

        Ok(Self {
            id,
            name,
            ingredients,
        })
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
        )
        .unwrap();

        assert_eq!(&hot_dog.id, SANDWICH_ID);
        assert_eq!(&hot_dog.name, SANDWICH_NAME);
        assert_eq!(ingredients.len(), hot_dog.ingredients.len());
        for (i, exp_ingr) in ingredients.iter().enumerate() {
            assert_eq!(exp_ingr, &hot_dog.ingredients[i]);
        }
    }

    #[test]
    fn should_fail_without_a_name_or_ingredients() {
        // idk what error type we gonna get right now
        let err_sandwich = Sandwich::new(
            "".to_string(),
            "".to_string(),
            vec!["Wurst".to_string(), "Ketchup".to_string()],
        );
        assert_eq!(err_sandwich.is_err(), true);
        assert_eq!(err_sandwich.unwrap_err(), "Any sandwich must have a name");

        let err_sandwich =
            Sandwich::new(SANDWICH_ID.to_string(), SANDWICH_NAME.to_string(), vec![]);

        assert_eq!(err_sandwich.is_err(), true);
        assert_eq!(
            err_sandwich.unwrap_err(),
            "Any sandwich must have at least an ingredient"
        );
    }
}
