use super::sandwich::{Sandwich, SandwichType};

#[derive(Debug)]
pub enum FindOneError {
    Unknown(String),
    NotFound,
}

pub fn find_one_sandwich<'a>(
    id: &'a str,
    name: &'a str,
    ingredients: &'a Vec<&str>,
) -> Result<Sandwich, FindOneError> {
    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    let sandwich = Sandwich::new(
        id.to_string(),
        name.to_string(),
        ingredients,
        SandwichType::Meat,
    )
    .unwrap();
    Ok(sandwich)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = &vec!["meat"];
        match find_one_sandwich("", "cheeseburger", ingredients) {
            Ok(s) => {
                assert_eq!(s.name().value(), "cheeseburger");
                assert_eq!(s.ingredients().value().len(), ingredients.len());
                for (i, exp_ingr) in s.ingredients().value().iter().enumerate() {
                    assert_eq!(exp_ingr, &ingredients[i]);
                }
            }
            Err(_) => unreachable!(),
        }
    }
}
