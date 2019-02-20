extern crate gl;
extern crate glutin;
extern crate rodio;

use glutin::dpi::LogicalSize;
use glutin::GlContext;
use rodio::Device;
use std::io::BufReader;
use std::io;
use std::fs;
use std::thread;
use std::sync::Arc;

fn handle_keypress(audio_device: Arc<Device>, input: glutin::KeyboardInput) {
    use glutin::ElementState::Released;
    
    if input.state == Released {
        return;
    }

    dbg!(input);

    let is_capital = input.modifiers.shift;

    let key = input.scancode; 
    let maybe_note = match key {
        11 => Some("48"),
        2 => Some("49"),
        3 => Some("50"),
        4 => Some("51"),
        5 => Some("52"),
        6 => Some("53"),
        7 => Some("54"),
        8 => Some("55"),
        9 => Some("56"),
        10 => Some("57"),
        16 => Some("81"),
        17 => Some("87"),
        18 => Some("69"),
        19 => Some("82"),
        20 => Some("84"),
        21 => Some("89"),
        22 => Some("85"),
        // TODO: Add the rest
        _ => None
    };

    if let Some(note) = maybe_note {
        let final_note = if is_capital {
            "b".to_owned() + note
        } else {
            "a".to_owned() + note
        };

        play_audio(audio_device, &final_note).ok();
    }
}

fn play_audio(audio_device: Arc<Device>, note: &str) -> Result<(), io::Error> {
    let filename = "sounds/".to_owned() + note + ".mp3";

    let file = fs::File::open(filename)?;

    thread::spawn(move || {
        rodio::play_once(&*audio_device, BufReader::new(file))
        .unwrap()
        .sleep_until_end()
    });
    Ok(())
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
