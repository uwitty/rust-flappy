use ggez::event;

pub mod game;
pub mod root;
pub mod title;

pub use game::Game;
pub use title::Title;

pub trait Scene: event::EventHandler {
    fn transit_to(&self) -> Option<&str>;
}
