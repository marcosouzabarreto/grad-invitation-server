use crate::db::connection;
use crate::models::{EventAttendance, EventAttendee};

pub async fn save_event_attendance(
    info: EventAttendance,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = connection::establish_connection().await;
    match client
        .execute(
            "INSERT INTO event_attendance (name, will_attend) VALUES ($1, $2)",
            &[&info.name, &info.will_attend],
        )
        .await
    {
        Ok(_) => println!("data saved successfully"),
        Err(e) => eprintln!("error = {}", e),
    };
    Ok(())
}

pub async fn get_event_attendees() -> Result<Vec<EventAttendee>, Box<dyn std::error::Error>> {
    let client = connection::establish_connection().await;
    let rows = client
        .query(
            "SELECT * FROM event_attendance WHERE will_attend IS TRUE",
            &[],
        )
        .await?;

    let attendees = rows
        .into_iter()
        .map(|row| EventAttendee {
            id: row.get(0),
            name: row.get(1),
            will_attend: row.get(2),
        })
        .collect();

    Ok(attendees)
}
