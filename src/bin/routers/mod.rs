// mod routers;
use rocket::http::RawStr;

// #[get("/user/:id")]
// pub fn getUserInfo() {}
// pub use user;

#[get("/user?wave&<id>")]
pub fn getUserInfo(id: &RawStr) -> String {
	format!("Hello, {}", id.as_str());
}
