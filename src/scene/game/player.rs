use ggez::graphics;
use ggez::nalgebra;
use ggez::timer;

use crate::graphics::SCREEN;

const GRAVITY: f32 = -9.8 * 3.0;
const FLAP_VELOCITY: f32 = 9.8 * 2.0;
const PIXELS_PER_METER: f32 = 64.0;

pub struct Player {
    pub image: graphics::Image,
    pub y: f32,
    pub flap_velocity: f32,
    pub start_falling: core::time::Duration,
}

impl Player {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Player> {
        Ok(Player {
            image: graphics::Image::new(ctx, "/player.png")?,
            y: 0.0,
            flap_velocity: 0.0,
            start_falling: ggez::timer::time_since_start(ctx),
        })
    }

    pub fn flap(&mut self, ctx: &mut ggez::Context) {
        self.flap_velocity = FLAP_VELOCITY;
        self.start_falling = timer::time_since_start(ctx);
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let delta = timer::delta(ctx);
        let v = self.flap_velocity
            + GRAVITY
                * (((timer::time_since_start(ctx) - self.start_falling).as_millis() as f32)
                    / 1000.0);
        self.y -= (v * (delta.as_millis() as f32)) / PIXELS_PER_METER;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let pos = nalgebra::Point2::new(
            SCREEN.width / 2.0 - f32::from(self.image.width()) / 2.0,
            self.y - f32::from(self.image.height()) / 2.0,
        );

        graphics::draw(
            ctx,
            &self.image,
            ggez::graphics::DrawParam::default().dest(pos),
        )
    }

    pub fn reset(&mut self, ctx: &mut ggez::Context) {
        self.y = 0.0;
        self.flap_velocity = 0.0;
        self.start_falling = ggez::timer::time_since_start(ctx);
    }
}
