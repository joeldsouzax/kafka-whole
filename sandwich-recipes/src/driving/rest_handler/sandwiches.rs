use crate::domain::sandwich::SandwichType;
use crate::helpers::string_vec_to_vec_str;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateSandwichRequest {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        message = "ingredients is required and must be at least 1 item"
    ))]
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichType,
}

pub async fn create_sandwich(
    request: Json<CreateSandwichRequest>,
) -> Result<Json<SandwichResponse>, ApiError> {
    validate(&request)?;
    let result = domain::create_sandwich::create_sandwich(
        &request.name,
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        &request.sandwich_type,
    )
    .await;

    result
        .map(|v| respond_json(SandwichResponse::from(v)))
        .map_err(|e| match e {
            CreateError::Unknown(m) => ApiError::Unknown(m),
            CreateError::InvalidData(m) => ApiError::InvalidData(m),
            CreateError::Conflict(m) => ApiError::Conflict(m),
        })?
}
