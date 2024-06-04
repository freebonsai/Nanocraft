use glfw::Window;

pub unsafe fn check_errors(check_point: &str) {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
        println!("###### GL ERROR ###### At: {}", check_point);
        let error_message = match error {
            gl::INVALID_ENUM => "GL_INVALID_ENUM: Enumeration parameter is not a legal enumeration!",
            gl::INVALID_VALUE => "GL_INVALID_VALUE: Value parameter is not a legal value for that function!",
            gl::INVALID_OPERATION => "GL_INVALID_OPERATION: Illegal state for that command!",
            gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
            gl::CONTEXT_LOST => "GL_CONTEXT_LOST",
            _ => "Unknown Error"
        };
        println!("{} : {}", error, error_message);
    }
}

pub fn framebuffer_size_callback(window: &mut Window, width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height) }
}