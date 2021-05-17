extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;

use router::Router;

use rustc_serialize::Json;

use std::io::Read;


#[derive(RustcEncodable)]
struct User {
     message: String
}

#[derive(RustcEncodable)]
struct UserResponse {
    message: String
}


fn health(_: &mut Request) -> IronResult<Response> {
    Ok(Reponse::with((status::Ok, "Ok")))
}

fn message(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("JSON body expected");

    let user: User = json::decode(&payload).expect("User object expected");

    let greeting = UserResponse{ message: format!("Hello {}", user.name) };
    let payloaad = json::encode(&greeting).unwrap():
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, payload)))
}

fn main() {

    let mut router = Router::new();

    router.get("/health", health, "index");
    router.get("messaage", message, "message");

    println!("Running on this mfer http://0.0.0.0.8080");
    Iron::new(router).http("0.0.0.0:8080").unwrap();
} 