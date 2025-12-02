use axum::{Json, extract::Query};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    first_name: String,
    last_name: String,
    id: Option<String>,
}

#[axum::debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    Json::from(Vehicle {
        manufacturer: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2022,
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}

// #[axum::debug_handler]
// pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
//     println!(
//         "Manufacturer: {0}, Model: {1}, Year: {2}",
//         v.manufacturer, v.model, v.year
//     );
//     v.id = Some(uuid::Uuid::new_v4().to_string());
//     Json::from(v)
// }

#[axum::debug_handler]
pub async fn vehicle_post(
    Query(mut v): Query<Vehicle>,
    Query(mut c): Query<Customer>,
) -> Json<Vehicle> {
    c.id = Some(uuid::Uuid::new_v4().to_string());
    print!(
        "ID:{0} Customer first name: {1} last name is {2}\n",
        c.id.unwrap(),
        c.first_name,
        c.last_name
    );
    //
    println!(
        "Manufacturer: {0}, Model: {1}, Year: {2}",
        v.manufacturer, v.model, v.year
    );
    v.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}
