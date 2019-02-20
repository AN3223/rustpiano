extern crate gl;
extern crate glutin;
extern crate rodio;

use glutin::dpi::LogicalSize;
use glutin::GlContext;
use rodio::Device;
use std::io::BufReader;
use std::fs;
use std::thread;
use std::sync::Arc;

fn handle_keypress(audio_device: Arc<Device>, input: glutin::KeyboardInput) {
    use glutin::VirtualKeyCode::*;
    use glutin::ElementState::Released;

    if input.state == Released {
        return;
    }

    let is_capital = input.modifiers.shift; // TODO: Support capital letters

    if let Some(key) = input.virtual_keycode {
        let maybe_note = match key {
            Key1 => Some("a49"),
            Key2 => Some("a50"),
            Key3 => Some("a51"),
            Key4 => Some("a52"),
            Key5 => Some("a53"),
            Key6 => Some("a54"),
            Key7 => Some("a55"),
            Key8 => Some("a56"),
            Key9 => Some("a57"),
            Key0 => Some("a48"),
            Q => Some("a81"),
            // TODO: Add the rest
            _ => None
        };

        if let Some(note) = maybe_note {
            play_audio(audio_device, note);
        }
    }
    
}

fn play_audio(audio_device: Arc<Device>, note: &str) {
    let filename = "sounds/".to_owned() + note + ".mp3";
    let file = fs::File::open(filename)
        .expect("failed to open sound file");

    thread::spawn(move || {
        rodio::play_once(&*audio_device, BufReader::new(file))
        .unwrap()
        .sleep_until_end()
    });
}

fn main() {
    let audio_device = Arc::new(rodio::default_output_device().unwrap());
    let mut events_loop = glutin::EventsLoop::new();

    let window = glutin::WindowBuilder::new()
        .with_title("Rustpiano")
        .with_dimensions(LogicalSize::new(500.0, 300.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        handle_keypress(Arc::clone(&audio_device), input);
                    }
                    glutin::WindowEvent::CloseRequested => running = false,
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}
