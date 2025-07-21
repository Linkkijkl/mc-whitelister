extern crate may_minihttp;

use std::io;
use may_minihttp::{HttpServer, HttpService, Request, Response};
use mc_rcon::RconClient;

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        
        let rcon_url = std::env::var("RCON_URL").expect("RCON_URL not set!");
        let rcon_password = std::env::var("RCON_PASSWORD").expect("RCON_PASSWORD not set!");

        if req.path() == "/" {
            rsp.body("Hello world!")
        } else {
            let params: Vec<&str> = req.path().split("/").collect();
            if let ["", username] = params[..] {
                if !username.chars().all(char::is_alphanumeric) {
                    rsp.status_code(400, "invalid username");
                    return Ok(());
                }
                let rcon = RconClient::connect(rcon_url);
                if rcon.is_err() {
                    rsp.status_code(500, "could not connect to minecraft server");
                    return Ok(());
                }
                let rcon = rcon.unwrap();

                if rcon.log_in(&rcon_password).is_err() {
                    rsp.status_code(500, "could not authenticate to minecraft server");
                    return Ok(());
                }
                if rcon.send_command(&format!("whitelist add {username}")).is_err() {
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
