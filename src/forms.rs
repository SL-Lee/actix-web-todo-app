use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(max = 32, message = "Username must not be longer than 32 characters."))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 32,
        message = "Password length must be between 8 and 32 characters."
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignupForm {
    #[validate(length(max = 32, message = "Username must not be longer than 32 characters."))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 32,
        message = "Password length must be between 8 and 32 characters."
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoEndpointData {
    #[validate(length(
        max = 100,
        message = "The length of the to-do title must not exceed 100 characters."
    ))]
    pub todo_title: String,

    #[validate(length(
        max = 512,
        message = "The length of the to-do contents must not exceed 512 characters."
    ))]
    pub todo_contents: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoEndpointData {
    pub todo_id: Uuid,

    #[validate(length(
        max = 100,
        message = "The length of the to-do title must not exceed 100 characters."
    ))]
    pub todo_title: String,

    #[validate(length(
        max = 512,
        message = "The length of the to-do contents must not exceed 512 characters."
    ))]
    pub todo_contents: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoStatusEndpointData {
    pub todo_id: Uuid,
    pub todo_completed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTodoEndpointData {
    pub todo_id: Uuid,
}
