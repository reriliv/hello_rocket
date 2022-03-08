#![feature(proc_macro_hygiene, decl_macro)]
// use hello_rocket::routers::user;
use rocket::config::Config;
// use crate::routers::getUserInfo;
mod routers;

// use rocket::Config;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Hello, rocket!"
}

fn main() {
	// routers
	// hello_rocket::
	// Config::get_string(&self, 'address');
	println!("development port: {}", Config::development().port);
	let config = Config::build(rocket::config::Environment::Staging)
		.port(8080)
		// .address("127.0.0.1")
		.unwrap();
	let app = rocket::custom(config);
	app.mount("/", routes![index]).launch();
	// let app = rocket::ignite();
	// app.mount("/", routes![index]);
	// app.
	/* app.mount("/user/:id", routes![routers::getUserInfo]);
	app.launch(); */
	// routers::getUserInfo
}
