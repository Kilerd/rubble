
use rocket_contrib::Json;

#[catch(404)]
pub fn not_found_catcher() -> String {
    "not found".to_string()
}

#[catch(401)]
pub fn unauthorized() -> Json {
    Json(json!({
        "message": "unauthorized"
    }))
}