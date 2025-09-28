use gage::window;
use gage::engine::types::{MessageQueue, EngineCvar};
use gage::colog;

use std::sync::{Arc, Mutex};


fn main() {
    colog::init();

    let mut cvar = EngineCvar::new();
    let mut message_queue = MessageQueue::new();
    let mut window = window::Window::new(
        800,
        600,
        String::from("gage_sandbox")
    );

    while window.running() {
        window.update(&mut message_queue);

        //Dispatch message
        while let Some(event) = message_queue.pop_front() {
            window.on_event(&event);
        }
    }
}
