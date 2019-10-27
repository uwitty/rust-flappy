use ggez::graphics;
use ggez::nalgebra;

use crate::graphics::SCREEN;

pub struct GameOver {
    pub text: graphics::Text,
    pub background: graphics::Mesh,
    pub instruction: graphics::Text,
}

impl GameOver {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<GameOver> {
        let text = GameOver::text(ctx)?;

        Ok(GameOver {
            text,
            background: GameOver::background(ctx)?,
            instruction: GameOver::instruction(ctx)?,
        })
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.background,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 0.0)),
        )?;

        crate::graphics::draw_centerized_text(
            ctx,
            &self.text,
            SCREEN.width / 2.0,
            (SCREEN.height / 3.0) * 1.0,
        )?;
        crate::graphics::draw_centerized_text(
            ctx,
            &self.instruction,
            SCREEN.width / 2.0,
            (SCREEN.height / 3.0) * 2.0,
        )
    }

    fn text(_ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Text> {
        let text = graphics::Text::new((
            graphics::TextFragment {
                text: "Game Over".to_string(),
                ..Default::default()
            },
            graphics::Font::default(),
            96.0,
        ));

        Ok(text)
    }

    fn background(ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Mesh> {
        graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(0.0, 0.0, SCREEN.width, SCREEN.height),
            graphics::Color::new(0.1, 0.1, 0.1, 1.0 / 2.0),
        )
    }

    fn instruction(_ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Text> {
        Ok(graphics::Text::new((
            graphics::TextFragment {
                text: "Push ENTER to restart".to_string(),
                ..Default::default()
            },
            graphics::Font::default(),
            32.0,
        )))
    }
}
