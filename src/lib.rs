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

use wasm_bindgen::prelude::wasm_bindgen;
use flask::{
    game_context::GameContext,
    palette,
};

use crate::game::Game;

pub const WINDOW_WIDTH: u32 = 256;
pub const WINDOW_HEIGHT: u32 = 256;
pub const FULLSCREEN: bool = false;

#[wasm_bindgen(start)]
pub fn start() {
    let game = Box::new(Game::new());

    if let Err(error) = GameContext::run(WINDOW_WIDTH, WINDOW_HEIGHT, FULLSCREEN, palette::flask_default(), game) {
        flask::log(format!("Flask error:\n{}", error).as_str());
    };
}
