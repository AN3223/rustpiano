extern crate gl;
extern crate glutin;
extern crate rodio;

use glutin::dpi::LogicalSize;
use glutin::GlContext;
use rustpiano::handle_keypress;

fn main() {
    let audio_device = rodio::default_output_device().unwrap();
    
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
                        handle_keypress(&audio_device, input);
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
