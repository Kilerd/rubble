use rocket::request::FromRequest;
use rocket::Request;
use rocket::request::Outcome;
use rocket::http::Status;
use rocket::outcome::Outcome::Success;
use rocket::outcome::Outcome::Failure;
use rocket::outcome::Outcome::Forward;

#[derive_FromForm]
#[derive(Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Admin {
    pub username: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let is_admin = request.cookies().get_private("LOG_ADMIN");
        let username = request.cookies().get_private("LOG_SESSION");

        if let Some(_flag) = is_admin {
            Success(Admin { username: username.unwrap().value().to_string() })
        }else {
            Failure((Status::Unauthorized, ()))
        }
    }
}