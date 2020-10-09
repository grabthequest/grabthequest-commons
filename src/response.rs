use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FusionAuthLoginResponseDTO {
    pub token: String,
    pub user: UserDTO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDTO {
    pub data: Option<FusionAuthLoginResponseDTO>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDTO {
    pub status_code: u16,
    pub description: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub active: bool,
    pub email: String,
    pub full_name: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationDTO {
    pub application_id: String,
    pub authentication_token: String,
    pub id: String,
    pub insert_instant: u64,
    pub last_login_instant: u64,
    pub last_update_instant: u64,
    pub timezone: String,
    pub username_status: String,
    pub verified: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FusionAuthRegisterResponseDTO {
    pub registration: RegistrationDTO,
    pub token: String,
    pub user: UserDTO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseDTO {
    pub data: Option<FusionAuthRegisterResponseDTO>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryDTO {
    pub user: UserDTO
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct GameDTO {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameResponseDTO {
    pub data: Option<GameDTO>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinGameResponseDTO {
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEventDTO {
    pub id: i32,
    pub game_id: i32,
    pub user_id: String,
    pub event_type: String,
    pub question_id: Option<i32>,
    pub score_percentage: Option<i32>,
    pub time_stamp: i64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEventResponseDTO {
    pub data: Option<Vec<GameEventDTO>>,
    pub error: Option<ErrorDTO>
}
