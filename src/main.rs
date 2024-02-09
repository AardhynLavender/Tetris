use crate::application::geometry::Vec2;
use crate::application::render::RendererProperties;
use crate::application::Application;

mod application;

pub fn main() -> Result<(), ()> {
    let mut application = Application::new(RendererProperties {
        title: String::from("Tetris"),
        dimensions: Vec2 { x: 1920, y: 1080 },
        fullscreen: false,
        show_cursor: false,
        opengl: true,
    });

    application.run()
}
