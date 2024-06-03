mod renderer;

const WINDOW_TITLE: &str = "Nanocraft";

extern crate glfw;

extern crate gl;

extern crate femtovg;

use glfw::{Action, Context, Key};
use femtovg::renderer::OpenGl;
use femtovg::{Color};
use crate::renderer::Renderer;

// https://github.com/rust-tutorials/learn-opengl/blob/main/examples/000-basic-window.rs

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(1200, 800, "Nanocraft", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    let opengl = unsafe { OpenGl::new_from_function(|s| window.get_proc_address(s) as *const _) }.unwrap();
    let renderer = &mut Renderer::create(opengl);

    // Loop until the user closes the window
    while !window.should_close() {
        let (width, height) = window.get_size();
        let w = width as u32;
        let h = height as u32;

        draw(renderer, w, h);

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

fn draw(renderer: &mut Renderer, w: u32, h: u32) {
    renderer.begin_frame(w, h);

    renderer.rect(0.0, 0.0, 100.0, 100.0, Color::rgb(255, 0, 0));

    renderer.end_frame();
}
