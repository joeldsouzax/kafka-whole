use super::sandwich::{Sandwich, SandwichType};

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
}

pub fn create_sandwich<'a>(
    name: &'a str,
    ingredients: &'a Vec<&str>,
    sandwich_type: &SandwichType,
) -> Result<Sandwich, CreateError> {
    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let sandwich = Sandwich::new(
        String::from(""),
        name.to_string(),
        ingredients,
        sandwich_type.clone(),
    )
    .map_err(|e| CreateError::InvalidData(e))?;

    Ok(sandwich)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::sandwich::SandwichType;

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = vec!["Wurst", "Ketchup"];
        let sandwich = create_sandwich("Hot dog", &ingredients, &SandwichType::Meat).unwrap();

        assert_eq!(sandwich.name().value(), "Hot dog");
        assert_eq!(sandwich.ingredients().value().len(), ingredients.len());

        for (i, exp_ingr) in sandwich.ingredients().value().iter().enumerate() {
            assert_eq!(exp_ingr, &ingredients[i]);
        }
    }
}
