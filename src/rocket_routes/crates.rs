use crate::{models::*, repositories::CrateRepository, rocket_routes::DbConn};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        // json!(e.to_string()) should be generic error like "Something went wrong"
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // json!(e.to_string()) should be generic error like "Something went wrong"
        CrateRepository::find_one(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(db: DbConn, new_crate: Json<NewCrate>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        // json!(e.to_string()) should be generic error like "Something went wrong"
        CrateRepository::create(c, new_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    db: DbConn,
    id: i32,
    a_crate: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // json!(e.to_string()) should be generic error like "Something went wrong"
        CrateRepository::update(c, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // json!(e.to_string()) should be generic error like "Something went wrong"
        CrateRepository::delete(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
