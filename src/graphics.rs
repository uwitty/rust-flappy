use ggez::graphics;
use ggez::nalgebra;

pub struct Screen {
    pub width: f32,
    pub height: f32,
}

pub const SCREEN: Screen = Screen {width: 640.0, height: 480.0};

pub fn draw_centerized_text(
    ctx: &mut ggez::Context,
    text: &graphics::Text,
    x: f32,
    y: f32,
) -> ggez::GameResult {
    let pos = nalgebra::Point2::new(
        x - (text.width(ctx) as f32) / 2.0,
        y - (text.height(ctx) as f32) / 2.0,
    );
    graphics::draw(ctx, text, graphics::DrawParam::default().dest(pos))
}
