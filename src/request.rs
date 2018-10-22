use rocket::request::FromRequest;
use rocket::Request;
use rocket::request::Outcome;
use rocket::http::Status;
use rocket::outcome::Outcome::Success;
use rocket::outcome::Outcome::Failure;
use rocket::outcome::Outcome::Forward;
use chrono::NaiveDateTime;
use rocket::config::Datetime;

#[derive_FromForm]
#[derive(Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Admin {
    pub id: i32,
    pub username: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let is_admin = request.cookies().get_private("LOG_ADMIN");
        let user_id = request.cookies().get_private("LOG_ID");
        let username = request.cookies().get_private("LOG_SESSION");

        if let Some(_flag) = is_admin {
            Success(Admin {
                username: username.unwrap().value().to_string(),
                id: user_id.unwrap().value().to_string().parse::<i32>().unwrap()
            })
        }else {
            Failure((Status::Unauthorized, ()))
        }
    }
}


#[derive_FromForm]
#[derive(Debug)]
pub struct ArticleEditForm {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_at: String,
    pub url: Option<String>,
}
