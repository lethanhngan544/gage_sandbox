use gage::renderer;
use gage::engine::types::{MessageQueue, EngineCvar, CVarValue, Message};
use gage::colog;
use gage::log;
use gage::glfw;


fn error_callback(error: glfw::Error, str: String) {
    log::error!("GLFW error: {:?}, {}", error, str);
}

fn main() {
    colog::init();

    let mut width = 1900;
    let mut height = 600;
    let title = "Sandbox !";

    let mut cvar = EngineCvar::new();
    let mut message_queue = MessageQueue::new();
    cvar.insert(String::from("window_width"), CVarValue::Int(width as i32));
    cvar.insert(String::from("window_height"), CVarValue::Int(height as i32));
    cvar.insert(String::from("window_title"), CVarValue::Str(String::from(title)));

    let mut glfw = glfw::init(error_callback).unwrap();

    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    let (mut window, reciever) = glfw.create_window(
            width as u32,
            height as u32,
            title,
            glfw::WindowMode::Windowed
        ).unwrap();
        window.set_all_polling(true);
    
    let mut renderer = renderer::Renderer::new(&window, &mut cvar);

    
    
    while !window.should_close() {

        //Update
        {
            glfw.poll_events();
            //Reciever events and dispatch to message bus
            for (_, event) in glfw::flush_messages(&reciever) {
                match event {
                    glfw::WindowEvent::CursorPos(x, y) => {
                        message_queue.push_back(Message::MouseMoved(x as u32, y as u32));
                    }
                    glfw::WindowEvent::FramebufferSize(width, height) => {
                        message_queue.push_back(Message::WindowResized(width as u32, height as u32));
                    }
                    glfw::WindowEvent::Key(Key, Scancode, Action, Modifiers) => {
                        if Action == glfw::Action::Press {
                            message_queue.push_back(Message::KeyPressed(Key as u32, Scancode as u32));
                        } else if Action == glfw::Action::Release {
                            message_queue.push_back(Message::KeyReleased(Key as u32, Scancode as u32));
                        }
                    }
                    _ => ()
                }
            }

            
            //Dispatch message
            while let Some(event) = message_queue.pop_front() {
                renderer.on_message(event.clone());
                if let Message::WindowResized(w, h) = event {
                    width = w;
                    height = h;
                }
            }
        }

        //Render
        {
            renderer.render(&cvar);
        }
        
    }
            
}


