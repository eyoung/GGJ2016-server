use std::sync::mpsc::{Sender, channel};
use std::thread;
use voodoo::Scene;
use voodoo::VoodooError;

pub struct GameManager {
    current_scene: Scene
}

impl GameManager {
    fn new() -> GameManager {
        GameManager {
            current_scene: Scene::new()
        }
    }
}

impl GameManager {
    pub fn run() -> Sender<VoodooMessage> {
        let (spawn_sender, spawn_receiver) = channel();
        let spawn_sender = spawn_sender.clone();
        thread::spawn(move || {
            let (event_sender, event_receiver) = channel();
            if let Ok(_) = spawn_sender.send(event_sender.clone()) {
                let manager = GameManager::new();
                while let Ok(message) = event_receiver.recv() {
                }
            }
        });
        spawn_receiver.recv().unwrap()
    }
}

pub enum VoodooMessage {
    Magic(Region, isize, Sender<String>)
}

pub enum Region {
    Head,
    Arms,
    Body
}

impl Region {
    pub fn new(text: &str) -> Result<Region, VoodooError> {
        match text {
            "head" => Ok(Region::Head),
            "arms" => Ok(Region::Arms),
            "body" => Ok(Region::Body),
            _ => Err(VoodooError::InvalidRegionError)
        }
    }
}