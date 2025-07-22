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
        if req.path() == "/" {
            // Index page
            rsp.header("content-type: text/html; charset=utf-8");
            let ctx = IndexTemplate {
                title: (*TITLE).clone(),
                map_url: (*MAP_URL).clone(),
            };
            let content = ctx.render_once().unwrap();
            let buffer_write_result = rsp.body_mut().write_str(&content);
            if buffer_write_result.is_err() {
                rsp.status_code(500, "buffer write error");
            }
        } else {
            // Api handling
            let params: Vec<&str> = req.path().split("/").collect();
            if let ["", username] = params[..] {
                if !username.chars().all(char::is_alphanumeric) {
                    rsp.status_code(400, "invalid username");
                    return Ok(());
                }
                let rcon = RconClient::connect(&*RCON_URL);
                if rcon.is_err() {
                    rsp.status_code(500, "could not connect to minecraft server");
                    return Ok(());
                }
                let rcon = rcon.unwrap();

                if rcon.log_in(&RCON_PASSWORD).is_err() {
                    rsp.status_code(500, "could not authenticate to minecraft server");
                    return Ok(());
                }
                if rcon
                    .send_command(&format!("whitelist add {username}"))
                    .is_err()
                {
                    rsp.status_code(500, "could not execute command");
                    return Ok(());
                }

                println!("Whitelisted {username}");
            } else {
                rsp.status_code(400, "bad input");
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
