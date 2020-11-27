use serde::{Serialize, Deserialize};

use crate::dto::*;

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
pub struct RegisterResponseDTO {
    pub data: Option<FusionAuthRegisterResponseDTO>,
    pub error: Option<ErrorDTO>
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
pub struct GameEventResponseDTO {
    pub data: Option<Vec<GameEventDTO>>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProblemResponseDTO {
    pub data: Option<ProblemViewDTO>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProblemsResponseDTO {
    pub data: Option<Vec<ProblemDTO>>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionTokenResponseDTO {
    pub token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionStatusResponseDTO {
    pub id: u16,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionDetailResponseDTO {
    pub token: String,
    pub stdout: String,
    pub time: String,
    pub memory: u32,
    pub message: String,
    pub status: SubmissionStatusResponseDTO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodeSubmissionResponseDTO {
    pub data: Option<CodeSubmissionDTO>,
    pub error: Option<ErrorDTO>
}
