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
use flask::rand::Rand;
use flask::renderer::Renderer;
use flask::input::Input;
use flask::palette::FlaskColor;
use flask::sprite::SpriteBank;

use crate::entity::Entity;
use crate::game::{WALK_AREA_MAX_X, WALK_AREA_MAX_Y};

const MAX_OFFSET_X: i64 = 30;
const MAX_OFFSET_Y: i64 = 10;
const PILLOW_SIZE: u32 = 10;

struct CloudPillow {
    position_x: f64,
    position_y: f64,
}

impl CloudPillow {
    pub fn new(rng: &mut Rand) -> CloudPillow {
        CloudPillow {
            position_x: rng.next_i64_in_range(-MAX_OFFSET_X, MAX_OFFSET_X) as f64,
            position_y: rng.next_i64_in_range(-MAX_OFFSET_Y, MAX_OFFSET_Y) as f64,
        }
    }
}

pub struct Cloud {
    position_x: f64,
    position_y: f64,
    speed: f64,
    cloud_pillows: Vec<CloudPillow>,
}

impl Cloud {
    pub fn new(rng: &mut Rand) -> Cloud {
        let mut cloud_pillows = vec![];
        for _ in 0..rng.next_i64_in_range(20, 30) {
            cloud_pillows.push(CloudPillow::new(rng))
        }

        Cloud {
            position_x: rng.next_i64_in_range(-WALK_AREA_MAX_X as i64, WALK_AREA_MAX_X as i64) as f64,
            position_y: rng.next_i64_in_range(0, WALK_AREA_MAX_Y as i64 + (MAX_OFFSET_Y + PILLOW_SIZE as i64)) as f64,
            speed: rng.next_i64_in_range(2, 5) as f64,
            cloud_pillows
        }
    }
}

impl Entity for Cloud {
    fn on_update(&mut self, _game_status: &mut GameStatus, renderer: &mut Renderer, _sprites: &mut SpriteBank, _input: &Input, delta_time: f64, game_speed: f64) {
        self.position_y -= self.speed * game_speed * delta_time;

        if self.position_y + ((MAX_OFFSET_Y + PILLOW_SIZE as i64) as f64) < 0.0 {
            self.position_y = WALK_AREA_MAX_Y + (MAX_OFFSET_Y + PILLOW_SIZE as i64) as f64;
        }

        for pillow in &mut self.cloud_pillows {
            renderer.circle_filled(self.position_x as i64 + pillow.position_x as i64, self.position_y as i64 + pillow.position_y as i64, PILLOW_SIZE, FlaskColor::White as u8);
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