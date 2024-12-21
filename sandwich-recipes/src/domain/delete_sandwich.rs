#[derive(Debug)]
pub enum DeleteError {
    Unknown(String),
}

pub fn delete_one_sandwich(id: &str) -> Result<(), DeleteError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_delete_a_sandwich() {
        match delete_one_sandwich("sand_id") {
            Ok(()) => {}
            _ => unreachable!(),
        }
    }
}
