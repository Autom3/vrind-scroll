extern crate quicksilver;

use quicksilver::{
    Result,
    combinators::result,
    geom::Vector,
    graphics::{Color, Font},
    lifecycle::{Asset, State, Window, run, Settings},
    prelude::Future,
};

pub mod common;
pub mod entities;
mod input;
mod renderer;

use entities::Player;

use input::player_controller;

use renderer::Background;
use renderer::Environment;
use renderer::FpsCounter;
use renderer::Renderer;

struct GameState {
    background: Background,
    environment: Environment,
    fps_counter: FpsCounter,
    player: Player,
    font: Asset<Font>,
}

impl State for GameState {
    fn new() -> Result<GameState> {
        let font = Asset::new(Font::load("fonts/big_shoulders/BigShouldersDisplay-Thin.ttf").and_then(|font| {
            result(Ok(font))
        }));

        Ok(GameState {
            background: Background::new(),
            environment: Environment::new(),
            fps_counter: FpsCounter::new(),
            player: Player::new(),
            font: font,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        player_controller::update(&mut self.player, window)?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.background.render(window, &mut self.font)?;
        self.environment.render(window, &mut self.font)?;
        self.fps_counter.render(window, &mut self.font)?;
        self.player.render(window, &mut self.font)?;
        Ok(())
    }
}

fn main() {
    run::<GameState>("vrind-scroll", Vector::new(1600, 900), Settings::default())
}
