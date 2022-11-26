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

use flask::game_status::GameStatus;
use flask::renderer::Renderer;
use flask::input::Input;
use flask::palette::FlaskColor;
use flask::sprite::SpriteBank;
use flask::rand::Rand;

use crate::entity::Entity;

const EXPLOSION_TIME: f64 = 5.0;
const MAX_OFFSET: i64 = 5;

pub struct Explosion {
    time_remaining: f64,
    position_x: f64,
    position_y: f64,
    rng: Rand,
}

impl Explosion {
    pub fn new(position_x: i64, position_y: i64) -> Explosion {
        Explosion {
            time_remaining: EXPLOSION_TIME,
            position_x: position_x as f64,
            position_y: position_y as f64,
            rng: Rand::new_with_seed((position_x + position_y) as u64),
        }
    }

    pub fn alive(&self) -> bool {
        self.time_remaining > 0.0
    }
}

impl Entity for Explosion {
    fn on_update(&mut self, _game_status: &mut GameStatus, renderer: &mut Renderer, _sprites: &mut SpriteBank, _input: &Input, delta_time: f64, game_speed: f64) {
        self.time_remaining -= game_speed  * delta_time;

        for _i in 0..10 {
            let offset_x = self.rng.next_i64_in_range(-MAX_OFFSET, MAX_OFFSET);
            let offset_y = self.rng.next_i64_in_range(-MAX_OFFSET, MAX_OFFSET);
            let color = match self.rng.next_bool() {
                true => FlaskColor::Red,
                false => FlaskColor::Yellow
            };

            renderer.circle_filled(self.position_x as i64 + offset_x, self.position_y as i64 + offset_y, 3, color as u8);
        }
    }

    fn x(&self) -> i64 {
        self.position_x as i64
    }

    fn y(&self) -> i64 {
        self.position_y as i64
    }

    fn collider_radius(&self) -> i64 {
        0
    }
}