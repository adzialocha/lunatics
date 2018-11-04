//! The smallest running game.

extern crate ggez;

use ggez::conf;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self};
use ggez::{Context, ContextBuilder, GameResult};

struct MainState {
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        println!("{}", keycode);
    }
}

pub fn main() {
    let cb = ContextBuilder::new("luncatics", "adzialocha")
        .window_setup(conf::WindowSetup::default().title("Lunatics"))
        .window_mode(conf::WindowMode::default().dimensions(240, 120));

    let ctx = &mut cb.build().unwrap();

    match MainState::new(ctx) {
        Err(e) => {
            println!("An error occurred: {}", e);
        }
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("An error occured: {}", e);
            } else {
                println!("Goodbye!");
            }
        }
    }
}
