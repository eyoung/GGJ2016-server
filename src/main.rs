mod voodoo;
extern crate hyper;
extern crate rustc_serialize;

use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::Read;
use hyper::server::{Handler, Server, Request, Response};
use voodoo::{GameManager, VoodooMessage, ActionContent};
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;

fn main() {
    let manager_sender = GameManager::run();
    Server::http("0.0.0.0:9282").unwrap().handle(NetworkHandler::new(manager_sender)).unwrap();
}

struct NetworkHandler {
    manager_sender: Mutex<Sender<VoodooMessage>>
}

impl NetworkHandler {
    fn new(sender: Sender<VoodooMessage>) -> NetworkHandler {
        NetworkHandler {
            manager_sender: Mutex::new(sender)
        }
    }
}

impl Handler for NetworkHandler {
    fn handle(&self, mut request: Request, response: Response) {
        let mut request_buffer = Vec::new();
        let mut response_body_channel = None;
        let request_body = request.read_to_end(&mut request_buffer);
        if let Ok(body_content) = String::from_utf8(request_buffer) {
            println!("Requested with body {}", &body_content);
            let result: Result<ActionContent, DecoderError> = json::decode(&body_content);
            if let Ok(action_content) = result {
                let (response_sender, response_receiver) = channel();
                let message = VoodooMessage::TurnAction(action_content, response_sender.clone());
                response_body_channel = Some(response_receiver);
                self.manager_sender.lock().unwrap().send(message).unwrap();
            }

        }
        if let Some(response_channel) = response_body_channel {
            let body = response_channel.recv().unwrap();
            response.send(format!("{}\n", body).as_bytes()).unwrap();
        } else {
            response.send(format!("{}\n", "it's alive").as_bytes()).unwrap();
        }
    }
}