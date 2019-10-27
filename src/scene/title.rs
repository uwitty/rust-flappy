use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;

use crate::graphics::SCREEN;

pub struct Title {
    title: graphics::Text,
    instruction: graphics::Text,
    entered: bool,
}

impl Title {
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<Title> {
        let title = graphics::Text::new((
            graphics::TextFragment {
                text: "Flappy Game".to_string(),
                ..Default::default()
            },
            graphics::Font::default(),
            96.0,
        ));

        let instruction = graphics::Text::new((
            graphics::TextFragment {
                text: "Push ENTER to start".to_string(),
                ..Default::default()
            },
            graphics::Font::default(),
            32.0,
        ));

        Ok(Title {
            title,
            instruction,
            entered: false,
        })
    }
}

impl super::Scene for Title {
    fn transit_to(&self) -> Option<&str> {
        if self.entered {
            Some("game")
        } else {
            None
        }
    }
}

impl event::EventHandler for Title {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, [0.4, 0.6, 0.9, 1.0].into());

        crate::graphics::draw_centerized_text(
            ctx,
            &self.title,
            SCREEN.width / 2.0,
            (SCREEN.height / 3.0) * 1.0,
        )?;
        crate::graphics::draw_centerized_text(
            ctx,
            &self.instruction,
            SCREEN.width / 2.0,
            (SCREEN.height / 3.0) * 2.0,
        )?;

        graphics::present(ctx)
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
    ) {
        if keycode == keyboard::KeyCode::Return {
            self.entered = true;
        }
    }
}
