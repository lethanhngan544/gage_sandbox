use gage::renderer;
use gage::engine::types::{MessageQueue, EngineCvar, CVarValue, Message};
use gage::colog;
use gage::log;
use gage::glfw;
use gage::uuid;

fn error_callback(error: glfw::Error, str: String) {
    log::error!("GLFW error: {:?}, {}", error, str);
}

fn main() {
    colog::init();

    let mut width = 1600;
    let mut height = 900;
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

    
    let mut model_id = uuid::Uuid::nil();
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
                            if Key == glfw::Key::B {
                                message_queue.push_back(Message::LoadStaticModel { path: String::from("assets/models/cube.obj") });
                            }
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
                renderer.on_message(event.clone(), &mut message_queue, &cvar);
                if let Message::StaticModelReady { id } = event {
                    model_id = id;
                }
                if let Message::WindowResized(w, h) = event {
                    width = w;
                    height = h;
                }
            }
        }

        //Render
        {
            if model_id != uuid::Uuid::nil() {
                renderer.add_draw_command(renderer::DrawCommands::StaticModel { id: model_id });
            }
            renderer.render(&cvar);
            renderer.clear_draw_commands();
        }
        
    }
            
}


