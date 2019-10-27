use ggez::event;
use ggez::input::keyboard;

use crate::scene;
use crate::graphics::SCREEN;

struct Root {
    pub scene: Box<crate::scene::Scene>,
}

impl Root {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Root> {
        let s = Root {
            scene: Box::new(scene::Title::new(ctx)?),
        };
        Ok(s)
    }
}

impl event::EventHandler for Root {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let Some("game") = self.scene.transit_to() {
            self.scene = Box::new(scene::Game::new(ctx)?);
        }

        self.scene.update(ctx)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.scene.draw(ctx)
    }

    fn key_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        keymods: keyboard::KeyMods,
    ) {
        self.scene.key_up_event(ctx, keycode, keymods)
    }
}

pub fn run() -> ggez::GameResult {

    let cb = ggez::ContextBuilder::new("super_simple", "flappy")
        .window_setup(ggez::conf::WindowSetup {
            title: "flappy".to_owned(),
            ..Default::default()
        })
        .window_mode(ggez::conf::WindowMode {
            width: SCREEN.width,
            height: SCREEN.height,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            resizable: false,
            ..Default::default()
        })
        .add_resource_path("resources");
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut Root::new(ctx)?;
    event::run(ctx, event_loop, state)
}
