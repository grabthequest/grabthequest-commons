use serde::{Serialize, Deserialize};

use crate::dto::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponseDTO {
    pub error: Option<ErrorDTO>
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
#[serde(rename_all = "camelCase")]
pub struct CodeSubmissionResponseDTO {
    pub data: Option<CodeSubmissionDTO>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSubmissionResultsResponseDTO {
    pub data: Option<Vec<TestCaseResultDTO>>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetGamePlayersResponseDTO {
    pub data: Option<Vec<GamePlayerDTO>>,
    pub error: Option<ErrorDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSubmissionEventsResponseDTO {
    pub data: Option<Vec<SubmissionEventDTO>>,
    pub error: Option<ErrorDTO>
}
