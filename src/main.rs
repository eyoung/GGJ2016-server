mod voodoo;
extern crate hyper;

use std::sync::mpsc::{channel, Sender, Receiver};
use hyper::server::{Handler, Server, Request, Response};
use voodoo::GameManager;

fn main() {
    GameManager::run();
}