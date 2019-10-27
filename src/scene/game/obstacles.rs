use ggez::graphics;
use ggez::nalgebra;
use ggez::timer;
use rand::Rng;

use crate::graphics::SCREEN;

const INTERVAL: f32 = SCREEN.width * 3.0 / 4.0;
const VELOCITY: f32 = INTERVAL / 4.0;
const GAP: f32 = 160.0;
const MARGIN: f32 = 10.0;
const NUM: usize = 4;

pub struct Obstacles {
    pub image1: graphics::Image,
    pub image2: graphics::Image,
    pub positions: [nalgebra::Point2<f32>; NUM],
    pub num_passed: u64,
}

impl Obstacles {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Obstacles> {
        let mut positions = [nalgebra::Point2::new(0.0, 0.0); NUM];
        Obstacles::reset_positions(&mut positions);

        Ok(Obstacles {
            image1: graphics::Image::new(ctx, "/obstacle1.png")?,
            image2: graphics::Image::new(ctx, "/obstacle2.png")?,
            positions,
            num_passed: 0,
        })
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let delta = (timer::delta(ctx).as_millis() as f32) / 1000.0;
        let threshold = SCREEN.width / 2.0;
        for position in &mut self.positions {
            let old_x = position.x;
            position.x -= delta * VELOCITY;

            if position.x < threshold && threshold < old_x {
                self.num_passed += 1;
            }
            if position.x < -f32::from(self.image1.width()) {
                position.x += INTERVAL * NUM as f32;
            }
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        for position in &self.positions {
            self.draw_with_offset(
                ctx,
                &self.image1,
                *position,
                -f32::from(self.image1.width()) / 2.0,
                GAP / 2.0,
            )?;
            self.draw_with_offset(
                ctx,
                &self.image2,
                *position,
                -f32::from(self.image2.width()) / 2.0,
                -GAP / 2.0 - f32::from(self.image2.height()),
            )?;
        }

        Ok(())
    }

    pub fn does_collide(&self, pos: (f32, f32), size: f32) -> bool {
        for position in &self.positions {
            if pos.0 + size / 2.0 > position.x - f32::from(self.image1.width()) / 2.0
                && pos.0 - size / 2.0 < position.x + f32::from(self.image1.width()) / 2.0
                && (pos.1 - size / 2.0 < position.y - GAP / 2.0
                    || pos.1 + size / 2.0 > position.y + GAP / 2.0)
            {
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        Obstacles::reset_positions(&mut self.positions);
        self.num_passed = 0;
    }

    fn draw_with_offset(
        &self,
        ctx: &mut ggez::Context,
        image: &graphics::Image,
        position: nalgebra::Point2<f32>,
        offset_x: f32,
        offset_y: f32,
    ) -> ggez::GameResult {
        let mut pos = position;
        pos.x += offset_x;
        pos.y += offset_y;

        graphics::draw(ctx, image, ggez::graphics::DrawParam::default().dest(pos))
    }

    fn reset_positions(positions: &mut [nalgebra::Point2<f32>; NUM]) {
        for (index, pos) in positions.iter_mut().enumerate() {
            pos.x = SCREEN.width / 2.0 + INTERVAL * (index + 1) as f32;
            pos.y = Obstacles::rand_y();
        }
    }

    fn rand_y() -> f32 {
        rand::thread_rng().gen_range(GAP / 2.0 + MARGIN, SCREEN.height - GAP / 2.0 - MARGIN)
    }
}
