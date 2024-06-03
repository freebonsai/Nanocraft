use femtovg::{Canvas, Color, Paint, Path};
use femtovg::renderer::OpenGl;


pub struct Renderer {
    canvas: Canvas<OpenGl>
}

// unsure if this should be "permanent"
#[allow(unused)]
impl Renderer {

    pub fn create(opengl: OpenGl) -> Renderer {
        let canvas = Canvas::new(opengl).expect("Cannot create canvas");
        Renderer {
            canvas
        }
    }

    pub fn begin_frame(&mut self, w: u32, h: u32) {
        self.canvas.set_size(w, h, 1.0);
        self.canvas.clear_rect(0, 0, w, h, Color::rgb(255, 255, 255));
    }

    pub fn end_frame(&mut self) {
        self.canvas.flush()
    }

    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let mut path = Path::new();
        path.rect(x, y, w, h);
        self.canvas.fill_path(&path, &Paint::color(color));
    }

    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
        let mut path = Path::new();
        path.rounded_rect(x, y, w, h, r);
        self.canvas.fill_path(&path, &Paint::color(color));
    }
}
