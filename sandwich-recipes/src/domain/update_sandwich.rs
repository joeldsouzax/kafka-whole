use super::sandwich::{Sandwich, SandwichType};

#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    NotFound,
    Conflict(String),
}

pub fn update_sandwich<'a>(
    id: &'a str,
    name: &'a str,
    ingredients: &'a Vec<&str>,
    sandwich_type: &SandwichType,
) -> Result<Sandwich, UpdateError> {
    if id.is_empty() {
        return Err(UpdateError::InvalidData(String::from(
            "Cannot update without a target id",
        )));
    }

    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let sandwich = Sandwich::new(
        String::from(id),
        name.to_string(),
        ingredients,
        sandwich_type.clone(),
    )
    .map_err(|e| UpdateError::InvalidData(e))?;

    Ok(sandwich)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_update_an_existing_sandwich() {
        let ingredients = vec!["ground meat", "cheese", "ketchup", "mayo"];
        let cheeseburger =
            update_sandwich("sand_id", "cheeseburger", &ingredients, &SandwichType::Meat).unwrap();

        assert_eq!(cheeseburger.name().value(), "cheeseburger");

        assert_eq!(cheeseburger.ingredients().value().len(), ingredients.len());
        for (i, exp_ingr) in cheeseburger.ingredients().value().iter().enumerate() {
            assert_eq!(exp_ingr, &ingredients[i]);
        }
    }
}
