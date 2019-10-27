use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra;

use super::game_over::GameOver;
use super::obstacles::Obstacles;
use super::player::Player;
use crate::graphics::SCREEN;

pub struct Game {
    player: Player,
    obstacles: Obstacles,
    game_over: GameOver,
    guide: graphics::Text,

    is_gameover: bool,
    show_guide: bool,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Game> {
        let guide = graphics::Text::new((
            graphics::TextFragment {
                text: "Push SPACE to flap!".to_string(),
                ..Default::default()
            },
            graphics::Font::default(),
            32.0,
        ));

        Ok(Game {
            player: Player::new(ctx)?,
            obstacles: Obstacles::new(ctx)?,
            game_over: GameOver::new(ctx)?,
            guide,
            is_gameover: false,
            show_guide: true,
        })
    }

    pub fn reset(&mut self, ctx: &mut ggez::Context) {
        self.player.reset(ctx);
        self.obstacles.reset();
        self.is_gameover = false;
    }

    fn draw_score(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let text = graphics::Text::new((
            graphics::TextFragment {
                text: format!("Score {:>5}", self.obstacles.num_passed),
                ..Default::default()
            },
            graphics::Font::default(),
            32.0,
        ));

        let pos = nalgebra::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &text, ggez::graphics::DrawParam::default().dest(pos))
    }

    fn draw_guide(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.show_guide {
            crate::graphics::draw_centerized_text(
                ctx,
                &self.guide,
                SCREEN.width / 2.0,
                SCREEN.height / 2.0,
            )
        } else {
            Ok(())
        }
    }
}

impl crate::scene::Scene for crate::scene::Game {
    fn transit_to(&self) -> Option<&str> {
        None
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.is_gameover {

        } else {
            self.obstacles.update(ctx)?;
            self.player.update(ctx)?;

            if self.obstacles.does_collide(
                (SCREEN.width / 2.0, self.player.y),
                f32::from(self.player.image.width()) / 2.0,
            ) {
                self.is_gameover = true;
            };
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, [0.4, 0.6, 0.9, 1.0].into());

        self.obstacles.draw(ctx)?;
        self.player.draw(ctx)?;

        if self.is_gameover {
            self.game_over.draw(ctx)?;
        } else {
            self.draw_guide(ctx)?;
        }

        self.draw_score(ctx)?;

        graphics::present(ctx)
    }

    #[allow(clippy::collapsible_if)]
    fn key_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
    ) {
        self.show_guide = false;

        if self.is_gameover {
            if keycode == keyboard::KeyCode::Return {
                self.reset(ctx);
            }
        } else {
            if keycode == keyboard::KeyCode::Space {
                self.player.flap(ctx);
            }
        }
    }
}
