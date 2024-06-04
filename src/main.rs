extern crate femtovg;
extern crate gl;
extern crate glfw;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::time::Instant;

use femtovg::Color;
use femtovg::renderer::OpenGl;
use gl::types::*;
use glfw::{Action, Context};
use glfw::WindowEvent::MouseButton;
use nalgebra::{Matrix4, Perspective3, Point3, Translation3, Vector3};

// use ogl33::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, glClear, glVertex3f};
use crate::renderer::Renderer;

mod renderer;

const WINDOW_TITLE: &str = "Nanocraft";

// https://github.com/rust-tutorials/learn-opengl/blob/main/examples/000-basic-window.rs

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();


    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));



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

    let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    let mut camera_position = Point3::new(0.0, 0.0, 0.0);

    let vertices: [f32; 108] = [
        // Positions
        1.0,  1.0, -1.0, // Top face
        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,

        1.0, -1.0,  1.0, // Bottom face
        -1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,

        1.0,  1.0,  1.0, // Front face
        -1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,

        1.0, -1.0, -1.0, // Back face
        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,

        -1.0,  1.0,  1.0, // Left face
        -1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0, -1.0, -1.0,

        1.0,  1.0, -1.0, // Right face
        1.0,  1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0, -1.0,  1.0,
    ];

    let colors: [f32; 108] = [
        // Colors
        0.0, 1.0, 0.0, // Top face (green)
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,

        1.0, 0.5, 0.0, // Bottom face (orange)
        1.0, 0.5, 0.0,
        1.0, 0.5, 0.0,
        1.0, 0.5, 0.0,
        1.0, 0.5, 0.0,
        1.0, 0.5, 0.0,

        1.0, 0.0, 0.0, // Front face (red)
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,

        1.0, 1.0, 0.0, // Back face (yellow)
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,

        0.0, 0.0, 1.0, // Left face (blue)
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,

        1.0, 0.0, 1.0, // Right face (magenta)
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
    ];

    // Set up vertex buffer and array objects
    let (mut vao, mut vbo_vertices, mut vbo_colors) = (0, 0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo_vertices);
        gl::GenBuffers(1, &mut vbo_colors);

        // Bind VAO
        gl::BindVertexArray(vao);

        // Bind vertex buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Vertex attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Bind color buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (colors.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            colors.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(1);

        // Unbind VAO
        gl::BindVertexArray(0);
    }

    // Enable depth testing
    unsafe { gl::Enable(gl::DEPTH_TEST); }
    // Set the depth function to less or equal (default)
    unsafe { gl::DepthFunc(gl::LEQUAL); }

    let mut last = Instant::now();
    let mut frames = 0;

    // Loop until the user closes the window
    while !window.should_close() {
        frames += 1;
        let now = Instant::now();
        // update every second
        if (now - last).as_secs_f32() >= 1.0 {
            window.set_title(&format!("Nanocraft; fps - {}", frames));
            frames = 0;
            last = now
        }

        // let (w, h) = window.get_size();
        //draw(renderer, w as u32, h as u32);

        // Clear the screen
        unsafe {
            gl::GetError();
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
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            let error = gl::GetError();
            println!("{}", error)
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
                glfw::WindowEvent::Key(glfw::Key::A, _, Action::Press, _) => {
                    camera_position.x -= 0.5
                }
                glfw::WindowEvent::Key(glfw::Key::D, _, Action::Press, _) => {
                    camera_position.x += 0.5
                }
                glfw::WindowEvent::Key(glfw::Key::Space, _, Action::Press, _) => {
                    camera_position.y += 0.5
                }
                glfw::WindowEvent::Key(glfw::Key::LeftShift, _, Action::Press, _) => {
                    camera_position.y -= 0.5
                }
                glfw::WindowEvent::Key(glfw::Key::S, _, Action::Press, _) => {
                    camera_position.z += 0.5
                }
                glfw::WindowEvent::Key(glfw::Key::W, _, Action::Press, _) => {
                    camera_position.z -= 0.5
                }

                MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
                    // let (x, y) = window.
                    println!("Clicked at")
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

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    void main() {
        gl_Position = projection * view * model * vec4(aPos, 1.0);
        ourColor = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 ourColor;

    void main() {
        FragColor = vec4(ourColor, 1.0);
    }
"#;



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