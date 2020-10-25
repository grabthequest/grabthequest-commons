use serde::{Serialize, Deserialize};

use crate::dto::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginDTO {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FusionAuthLoginRequestDTO {
    pub login_id: String,
    pub password: String,
    pub application_id: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FusionAuthRegisterRequestDTO {
    pub skip_registration_verification: bool,
    pub skip_verification: bool,
    pub generate_authentication_token: bool,
    pub registration: FusionAuthRegistrationAdditionDTO,
    pub user: FusionAuthUserRegistrationDTO,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateProblemRequestDTO {
    pub problem: ProblemDTO
}
