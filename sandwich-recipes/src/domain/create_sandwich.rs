#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = vec!["Wurst", "Ketchup"];
        let sandwich = create_sandwich("Hot dog", &ingredients, &SandwichType::Meat).unwrap();
        assert_eq!(sandwich.name().value(), "Hot dog");

        assert_eq!(sandwich.ingredients().value().len(), ingredients.len());
        for (i, exp_ingr) in sandwich.ingredients().value.iter().enumerate() {
            assert_eq!(exp_ingr, &ingredients[i]);
        }
    }
}
