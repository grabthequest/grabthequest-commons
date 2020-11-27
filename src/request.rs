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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitCodeRequestDTO {
    pub code: String,
    pub language_id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Judge0SubmissionRequestDTO {
    pub source_code: String,
    pub language_id: u32,
    pub stdin: String,
    pub expected_output: String,
    pub cpu_time_limit: f32,
    pub cpu_extra_time: f32, // always 0.5 s
}
