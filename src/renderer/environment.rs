use quicksilver::{
    Result,
    geom::{Rectangle, Circle, Line, Triangle, Transform},
    graphics::{Background::Col, Font, Color},
    lifecycle::{Asset, Window}
};

use super::Renderer;

pub struct Environment {}

impl Environment {
    pub fn new() -> Self {
        Environment {}
    }
}

impl Renderer for Environment {
    fn render(&mut self, window: &mut Window, _: &mut Asset<Font>) -> Result<()> {

        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        window.draw(&Circle::new((400, 300), 100), Col(Color::YELLOW));

        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::GREEN), Transform::rotate(45), 10);

        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );

        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );

        Ok(())
    }
}
