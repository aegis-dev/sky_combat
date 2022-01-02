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

use rand::Rng;

use flask::game_status::GameStatus;
use flask::renderer::Renderer;
use flask::input::{Input, Key, State};
use flask::sprite::{SpriteBank, SpriteID};
use flask::palette::FlaskColor;

use crate::entity::Entity;
use crate::game::{WALK_AREA_MAX_X, WALK_AREA_MAX_Y};

const SHOOT_INTERVAL: f64 = 1.0;

pub struct Player {
    health: u8,
    speed: f64,
    position_x: f64,
    position_y: f64,
    shoot_timer: f64,
    sprite_id: SpriteID,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 3,
            speed: 10.0,
            position_x: 0.0,
            position_y: 25.0,
            shoot_timer: SHOOT_INTERVAL,
            sprite_id: SpriteID(0)
        }
    }

    pub fn damage(&mut self) {
        if self.health >= 1 {
            self.health -= 1;
        }
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn alive(&self) -> bool {
        self.health > 0
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_timer < 0.0
    }

    pub fn reset_shoot_interval(&mut self) {
        self.shoot_timer = SHOOT_INTERVAL;
    }
}

impl Entity for Player {
    fn on_update(&mut self, _game_status: &mut GameStatus, renderer: &mut Renderer, sprites: &mut SpriteBank, input: &Input, delta_time: f64, game_speed : f64) {
        self.shoot_timer -= game_speed * delta_time;

        if input.get_key_state(Key::D) == State::Down {
            self.position_x += self.speed * game_speed * delta_time;
        } else if input.get_key_state(Key::A) == State::Down {
            self.position_x -= self.speed * game_speed * delta_time;
        }

        // Don't let overflow
        if self.position_x > WALK_AREA_MAX_X {
            self.position_x = WALK_AREA_MAX_X;
        } else if self.position_x < -WALK_AREA_MAX_X {
            self.position_x = -WALK_AREA_MAX_X;
        }

        if input.get_key_state(Key::W) == State::Down {
            self.position_y += self.speed * game_speed * delta_time;
        } else if input.get_key_state(Key::S) == State::Down {
            self.position_y -= self.speed * game_speed * delta_time;
        }

        // Don't let overflow
        if self.position_y < 0.0 {
            self.position_y = 0.0;
        } else if self.position_y > WALK_AREA_MAX_Y {
            self.position_y = WALK_AREA_MAX_Y;
        }

        let sprite = sprites.get_sprite(&self.sprite_id).unwrap();

        let x_offset = (sprite.get_width() / 2) as i64;
        let y_offset = (sprite.get_height() / 2) as i64;

        renderer.sprite(sprite, self.position_x as i64 - x_offset, self.position_y as i64 - y_offset, false);

        renderer.line(
            self.position_x as i64 - 2,
            self.position_y as i64 - 9,
            self.position_x as i64 - 2,
            self.position_y as i64 - 9 - rand::thread_rng().gen_range(0..5),
            FlaskColor::Red as u8
        );
        renderer.line(
            self.position_x as i64 - 1,
            self.position_y as i64 - 9,
            self.position_x as i64 - 1,
            self.position_y as i64 - 9 - rand::thread_rng().gen_range(2..7),
            FlaskColor::Yellow as u8
        );
        renderer.line(
            self.position_x as i64,
            self.position_y as i64 - 9,
            self.position_x as i64,
            self.position_y as i64 - 9 - rand::thread_rng().gen_range(0..5),
            FlaskColor::Red as u8
        );
    }

    fn x(&self) -> i64 {
        self.position_x as i64
    }

    fn y(&self) -> i64 {
        self.position_y as i64
    }

    fn collider_radius(&self) -> i64 {
        3
    }
}