//
// Copyright © 2021-2022  Egidijus Lileika
//
// This file is part of Sky Combat - Sample game of Flask game framework
//
// Sky Combat is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Sky Combat is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Sky Combat. If not, see <https://www.gnu.org/licenses/>.
//

mod game;
mod entity;
mod player;
mod enemy;
mod projectile;
mod game_over;
mod explosion;
mod cloud;

use flask::{
    game_context::GameContext,
    palette,
};

use crate::game::Game;

pub const WINDOW_WIDTH: u32 = 256;
pub const WINDOW_HEIGHT: u32 = 256;

fn main() {
    let mut game_context = GameContext::new();

    let game = Box::new(Game::new());

    match game_context.run(WINDOW_WIDTH, WINDOW_HEIGHT, "Sky Combat", palette::flask_default(), game) {
        Ok(_) => { }
        Err(error) => println!("{}", error)
    };
}
