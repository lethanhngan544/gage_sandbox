use gage::window;
use gage::engine::state::State;
use gage::engine::bus::Bus;
use gage::colog;

use std::sync::{Arc, Mutex};


fn main() {
    colog::init();

    let mut state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    let mut bus: Arc<Mutex<Bus>> = Arc::new(Mutex::new(Bus::new()));
    let mut window = window::Window::new(
        800,
        600,
        String::from("gage_sandbox")
    );
    window.register(state.clone(), bus.clone());

    while window.running() {
        window.update(state.clone(), bus.clone());

        bus.lock().unwrap().dispatch();
    }
}
