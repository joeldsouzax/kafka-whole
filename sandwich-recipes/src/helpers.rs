use crate::driving::rest_handler::errors::ApiError;
use actix_web::web::Json;
use serde::Serialize;

pub fn string_vec_to_vec_str(list: &Vec<String>) -> Vec<&str> {
    let mut pointer_list = Vec::<&str>::new();
    for item in list {
        pointer_list.push(item);
    }
    pointer_list
}

pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}
