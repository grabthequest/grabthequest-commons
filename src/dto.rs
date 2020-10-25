use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FusionAuthLoginResponseDTO {
    pub token: String,
    pub user: UserDTO
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
pub struct UserQueryDTO {
    pub user: UserDTO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEventDTO {
    pub id: i32,
    pub game_id: i32,
    pub user_id: String,
    pub user_name: String,
    pub event_type: String,
    pub question_id: Option<i32>,
    pub score_percentage: Option<i32>,
    pub timestamp: i64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProblemDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub test_cases: BTreeSet<TestCaseDTO>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestCaseDTO {
    pub id: i32,
    pub problem_id: i32,
    pub seq_no: i32,
    pub input: String,
    pub output: String
}

impl PartialEq for TestCaseDTO {
    fn eq(&self, other: &TestCaseDTO) -> bool {
        return self.id == other.id;
    }
}

impl Eq for TestCaseDTO {}

impl PartialOrd for TestCaseDTO {
    fn partial_cmp(&self, other: &TestCaseDTO) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for TestCaseDTO {
    fn cmp(&self, other: &TestCaseDTO) -> Ordering {
        return self.seq_no.cmp(&other.seq_no);
    }
}
