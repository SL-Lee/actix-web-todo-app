use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoEndpointData {
    pub todo_title: String,
    pub todo_contents: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoEndpointData {
    pub todo_id: i32,
    pub todo_title: String,
    pub todo_contents: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoStatusEndpointData {
    pub todo_id: i32,
    pub todo_completed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTodoEndpointData {
    pub todo_id: i32,
}
