mod renderer;

const WINDOW_TITLE: &str = "Nanocraft";

extern crate glfw;

extern crate gl;

extern crate femtovg;

use femtovg::renderer::OpenGl;
use femtovg::{Color};
use glfw::WindowEvent::{MouseButton};
// use ogl33::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, glClear, glVertex3f};
use crate::renderer::Renderer;

// https://github.com/rust-tutorials/learn-opengl/blob/main/examples/000-basic-window.rs

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(1200, 800, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_mouse_button_polling(true);
    window.set_key_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    let opengl = unsafe { OpenGl::new_from_function(|s| window.get_proc_address(s) as *const _) }.unwrap();
    let renderer = &mut Renderer::create(opengl);

    // Loop until the user closes the window
    while !window.should_close() {
        let (w, h) = window.get_size();
        //draw(renderer, w as u32, h as u32);

        // Clear the screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Use shader program
        unsafe {
            gl::UseProgram(shader_program);

            // Set up camera transformation
            // Set up camera transformation
            let view = Matrix4::look_at_rh(&camera_position, &(camera_position + Vector3::new(0.0, 0.0, -1.0)), &Vector3::new(0.0, 1.0, 0.0));
            let view_location = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_ptr());

            // Model matrix remains the same
            let translation = Translation3::new(1.5, 0.0, -7.0);
            let model: Matrix4<f32> = Matrix4::<f32>::identity() * translation.to_homogeneous();
            let model_location = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model.as_ptr());

            // Projection matrix remains the same
            let projection = Perspective3::new(800.0 / 600.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
            let projection_location = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());

            // Draw cube
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::QUADS, 0, 24);
        }


        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => {
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

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0; len as usize];
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}