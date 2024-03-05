use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendance {
    pub name: String,
    pub will_attend: bool,
}

#[derive(Serialize)]
pub struct EventAttendee {
    pub id: i32,
    pub name: String,
    pub will_attend: bool,
}
