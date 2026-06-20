use include_dir_macro::include_dir;
use lazy_static::lazy_static;
use may_minihttp::{HttpServer, HttpService, Request, Response};
use mc_rcon::RconClient;
use sailfish::TemplateSimple;
use std::{
    fmt::Write,
    io,
    path::Path,
};

#[derive(TemplateSimple)]
#[template(path = "index.stpl")]
struct IndexTemplate<'a> {
    title: &'a str,
    map_url: &'a str,
    info: &'a str,
    username_label: &'a str,
    password_label: &'a str,
    submit_label: &'a str,
}

lazy_static! {
    static ref RCON_URL: String = std::env::var("RCON_URL").expect("RCON_URL not set!");
    static ref RCON_PASSWORD: String =
        std::env::var("RCON_PASSWORD").expect("RCON_PASSWORD not set!");
    static ref WHITELIST_PASSWORD: String =
        std::env::var("WHITELIST_PASSWORD").expect("WHITELIST_PASSWORD not set!");
    static ref TITLE: String = std::env::var("TITLE").unwrap_or_else(|_| String::from("Minecraft server"));
    static ref MAP_URL: String = std::env::var("MAP_URL").unwrap_or_default();
    static ref INFO: String = std::env::var("INFO").unwrap_or_default();
    static ref USERNAME_LABEL: String = std::env::var("USERNAME_LABEL").unwrap_or_else(|_| String::from("Minecraft username:"));
    static ref PASSWORD_LABEL: String = std::env::var("PASSWORD_LABEL").unwrap_or_else(|_| String::from("Whitelist password:"));
    static ref SUBMIT_LABEL: String = std::env::var("SUBMIT_LABEL").unwrap_or_else(|_| String::from("Submit"));
}

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        // Api handling
        let params: Vec<&str> = req.path().split("/").skip(1).collect();
        match params[..] {
            // Index page
            [""] => {
                // Render page from template
                let ctx = IndexTemplate {
                    title: &TITLE,
                    map_url: &MAP_URL,
                    info: &INFO,
                    username_label: &USERNAME_LABEL,
                    password_label: &PASSWORD_LABEL,
                    submit_label: &SUBMIT_LABEL,
                };
                let content = ctx.render_once().unwrap();

                // Return rendered page
                let buffer_write_result = rsp.body_mut().write_str(&content);
                if buffer_write_result.is_err() {
                    rsp.status_code(500, "internal server error");
                    rsp.body("buffer write error")
                } else {
                    rsp.header("Content-Type: text/html; charset=utf-8");
                }
            }

            // Whitelist api route
            ["api", "whitelist", password, username] => {
                if password != *WHITELIST_PASSWORD {
                    rsp.status_code(400, "bad request");
                    rsp.body("invalid whitelist password");
                    return Ok(());
                }
                if !username.chars().all(|a| { char::is_alphanumeric(a) || a == '_' }) {
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

                // TODO: Better logging
                println!("Whitelisted {username}");
            }

            // Static files
            [filename, ..] => {
                let static_content = include_dir!("static");
                let path = Path::new(filename);
                match static_content.get(path) {
                    Some(content) => {
                        rsp.body_vec(content.to_owned().to_owned());
                        let extension = path.extension().map(|a| a.to_str());
                        let extension = if let Some(Some(extension)) = extension {
                            extension
                        } else {
                            ""
                        };
                        match extension {
                            "js" => rsp.header("Content-Type: text/javascript"),
                            "css" => rsp.header("Content-Type: text/css"),
                            "webp" => rsp.header("Content-Type: image/webp"),
                            "otf" => rsp.header("Content-Type: font/otf"),
                            _ => rsp.header("Content-Type: text/plain"),
                        };
                    }
                    None => {
                        // 404
                        rsp.status_code(404, "not found");
                        rsp.body("resource_not_found");
                    }
                }
            }

            _ => unreachable!()
        }

        Ok(())
    }
}

// Start the server in `main`.
fn main() {
    let server = HttpServer(HelloWorld).start("0.0.0.0:8080").unwrap();
    server.join().unwrap();
}
