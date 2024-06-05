extern crate femtovg;
extern crate gl;
extern crate glfw;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::time::Instant;

use femtovg::Color;
use gl::types::*;
use glfw::{Action, Context, Key};
use glfw::WindowEvent::MouseButton;
use nalgebra::{Matrix4, Perspective3, Translation3, Vector3};

use crate::gl_handler::{check_errors, framebuffer_size_callback};
// use ogl33::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, glClear, glVertex3f};
use crate::renderer::Renderer;

mod renderer;
mod gl_handler;
mod camera;
mod cube;

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
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    // let opengl = unsafe { OpenGl::new_from_function(|s| window.get_proc_address(s) as *const _) }.unwrap();
    // let renderer = &mut Renderer::create(opengl);

    let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    let mut camera = camera::Camera::new(Vector3::new(0.0, 0.0, 0.0), 0.0, 0.0, 1.0);

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
            (cube::VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            cube::VERTICES.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Vertex attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Bind color buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (cube::COLORS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            cube::COLORS.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(1);

        // Unbind VAO
        gl::BindVertexArray(0);
    }

    unsafe {
        window.set_framebuffer_size_callback(framebuffer_size_callback);
        // Enable depth testing
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut z: i8 = 0;
    let mut fly_speed: f32 = 1.0;

    let mut last_x = 400.0;
    let mut last_y = 300.0;
    let mut first_mouse = true;

    let mut last_update = Instant::now();
    let mut last_frame = Instant::now();
    let mut frames = 0;

    // Loop until the user closes the window
    while !window.should_close() {
        frames += 1;
        let now = Instant::now();
        let duration_since = now.duration_since(last_frame).as_secs_f32();
        last_frame = Instant::now();
            // update every second
        if now.duration_since(last_update).as_secs_f32() >= 1.0 {
            window.set_title(&format!("Nanocraft; fps - {}", frames));
            frames = 0;
            last_update = now
        }

        let delta = duration_since;

        camera.process_keyboard(camera::Direction::X, x as f32 * fly_speed, delta); // works
        camera.process_keyboard(camera::Direction::Z, z as f32 * fly_speed, delta);
        camera.position.y += (y as f32 * fly_speed) * delta;


        struct Cube {
            position: Vector3<f32>,
            color: Vector3<f32>,
        }

        let cubes = vec![
            Cube {
                position: Vector3::new(1.5, 0.0, -7.0),
                color: Vector3::new(1.0, 0.0, 0.0),
            },
            Cube {
                position: Vector3::new(8.0, 0.0, -7.0),
                color: Vector3::new(0.0, 1.0, 0.0),
            },
            Cube {
                position: Vector3::new(8.0, 10.0, -7.0),
                color: Vector3::new(0.0, 1.0, 1.0),
            },
            Cube {
                position: Vector3::new(-5.0, 0.0, 7.0),
                color: Vector3::new(0.5, 0.5, 0.0),
            },
            // Add more cubes as needed
        ];


        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        // Calculate the view matrix using look_at_rh
        let view = camera.view_matrix();

        // Use shader program
        unsafe {
            gl::UseProgram(shader_program);

            // Set the view matrix uniform
            let view_location = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_ptr());

            // Set the model matrix (unchanged)
            let translation = Translation3::new(1.5, 0.0, -7.0);
            let model: Matrix4<f32> = Matrix4::<f32>::identity() * translation.to_homogeneous();
            let model_location = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model.as_ptr());

            // Set the projection matrix (unchanged)
            let projection = Perspective3::new(800.0 / 600.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
            let projection_location = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());

            for cube in &cubes {
                // Set the model matrix uniform
                let translation = Translation3::new(cube.position.x, cube.position.y, cube.position.z);
                let model: Matrix4<f32> = Matrix4::<f32>::identity() * translation.to_homogeneous();
                let model_location = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model.as_ptr());

                // Set the color uniform
                let uniform_color_location = gl::GetUniformLocation(shader_program, CString::new("uniformColor").unwrap().as_ptr());
                gl::Uniform3f(uniform_color_location, cube.color.x, cube.color.y, cube.color.z);

                // Bind VAO and draw
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // Check for errors
            check_errors("Post Draw!");
        }


        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match (key, action) { // every frame it moves based on these values
                        (Key::A, Action::Press) | (Key::D, Action::Release) => x += 1,
                        (Key::A, Action::Release) | (Key::D, Action::Press) => x -= 1,
                        (Key::W, Action::Press) | (Key::S, Action::Release) => z -= 1,
                        (Key::W, Action::Release) | (Key::S, Action::Press) =>  z += 1,
                        (Key::Space, Action::Press) | (Key::LeftShift, Action::Release) => y += 1,
                        (Key::Space, Action::Release) | (Key::LeftShift, Action::Press) => y -= 1,
                        _ => {}
                    }
                }

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    if first_mouse {
                        last_x = xpos;
                        last_y = ypos;
                        first_mouse = false;
                    }

                    let xoffset = xpos - last_x;
                    let yoffset = last_y - ypos; // Reversed since y-coordinates range from bottom to top
                    last_x = xpos;
                    last_y = ypos;

                    let sensitivity = 0.1; // Change this value to your liking
                    camera.yaw += xoffset as f32 * sensitivity;
                    camera.pitch += yoffset as f32 * sensitivity;

                    // Constrain the pitch to avoid flipping the camera
                    if camera.pitch > 89.0_f32 {
                        camera.pitch = 89.0_f32;
                    }
                    if camera.pitch < -89.0_f32 {
                        camera.pitch = -89.0_f32;
                    }
                }

                MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
                    let (x, y) = window.get_cursor_pos();
                    println!("Clicked at x: {}, y: {}", x, y)
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

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    void main() {
        gl_Position = projection * view * model * vec4(aPos, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 ourColor;
    uniform vec3 uniformColor;

    void main() {
        FragColor = vec4(uniformColor, 1.0);
    }
"#;

// i think we make placing + breaking or atleast having a easy way to make multiple cubes with like different colors

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