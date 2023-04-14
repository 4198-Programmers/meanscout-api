use rocket::Request;
use crate::warning;
use std::io::Write;

#[catch(default)]
pub fn default_catcher(req: &Request) -> String {
    warning!(format!("Unknown Code: {} attempted {}",req.client_ip().unwrap(), req.uri()));
    format!("Something sure happened when you went to '{}'. Try something else?", req.uri())
}

#[catch(400)]
pub fn bad_request(req: &Request) -> String {
    warning!(format!("400: Bad Request \n  > ip: {}\n  > uri: {}",req.client_ip().unwrap(), req.uri()));
    format!("Bad request to '{}, try again later or change data'", req.uri())
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    warning!(format!("404: Not Found \n  > ip: {}\n  > uri: {}",req.client_ip().unwrap(), req.uri()));
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(413)]
pub fn content_too_large(req: &Request) -> String {
    warning!(format!("413: Content Too Large\n  > ip: {}\n  > uri: {}", req.client_ip().unwrap(), req.uri()));
    format!("hehe")
}

#[catch(418)]
pub fn im_a_teapot(req: &Request) -> String {
    warning!(format!("418: I'm a teapot. How."));
    format!("I don't know how you did this.")
}