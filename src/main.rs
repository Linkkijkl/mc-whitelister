extern crate may_minihttp;

use lazy_static::lazy_static;
use may_minihttp::{HttpServer, HttpService, Request, Response};
use mc_rcon::RconClient;
use sailfish::TemplateSimple;
use std::{fmt::Write, io};

#[derive(TemplateSimple)]
#[template(path = "index.stpl")]
struct IndexTemplate {
    title: String,
    map_url: String,
}

lazy_static! {
    static ref RCON_URL: String = std::env::var("RCON_URL").expect("RCON_URL not set!");
    static ref RCON_PASSWORD: String =
        std::env::var("RCON_PASSWORD").expect("RCON_PASSWORD not set!");
    static ref WHITELIST_PASSWORD: String =
        std::env::var("WHITELIST_PASSWORD").expect("WHITELIST_PASSWORD not set!");
    static ref TITLE: String = std::env::var("TITLE").unwrap_or_default();
    static ref MAP_URL: String = std::env::var("MAP_URL").unwrap_or_default();
}

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        // Api handling
        let params: Vec<&str> = req.path().split("/").collect();
        match params[..] {
            // Index page
            ["", ""] => {
                // Render page from template
                let ctx = IndexTemplate {
                    title: (*TITLE).clone(),
                    map_url: (*MAP_URL).clone(),
                };
                let content = ctx.render_once().unwrap();

                // Return rendered page
                let buffer_write_result = rsp.body_mut().write_str(&content);
                if buffer_write_result.is_err() {
                    rsp.status_code(500, "buffer write error");
                }
                rsp.header("content-type: text/html; charset=utf-8");
            }

            // JavaScript
            ["", "main.js"] => {
                rsp.header("content-type: text/javascript");
                rsp.body(include_str!("../static/main.js"));
            }

            // Whitelist api route
            ["", "api", "whitelist", password, username] => {
                if password != *WHITELIST_PASSWORD {
                    rsp.status_code(400, "bad request");
                    rsp.body("invalid whitelist password");
                    return Ok(());
                }
                if !username.chars().all(char::is_alphanumeric) {
                    rsp.status_code(400, "bad request");
                    rsp.body("invalid username");
                    return Ok(());
                }
                let rcon = RconClient::connect(&*RCON_URL);
                if rcon.is_err() {
                    rsp.status_code(500, "internal server error");
                    rsp.body("could not connect to minecraft server");
                    return Ok(());
                }
                let rcon = rcon.unwrap();

                if rcon.log_in(&RCON_PASSWORD).is_err() {
                    rsp.status_code(500, "internal server error");
                    rsp.body("could not authenticate to minecraft server");
                    return Ok(());
                }
                if rcon
                    .send_command(&format!("whitelist add {username}"))
                    .is_err()
                {
                    rsp.status_code(500, "internal server error");
                    rsp.body("could not execute command");
                    return Ok(());
                }

                println!("Whitelisted {username}");
            }

            // 404
            _ => {
                rsp.status_code(404, "not found");
                rsp.body("resource not found");
            }
        }

        Ok(())
    }
}

// Start the server in `main`.
fn main() {
    let server = HttpServer(HelloWorld).start("0.0.0.0:8080").unwrap();
    server.join().unwrap();
}
