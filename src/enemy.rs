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
use flask::input::Input;
use flask::sprite::{SpriteBank, SpriteID};
use flask::palette::FlaskColor;

use crate::entity::Entity;
use crate::game::WALK_AREA_MAX_X;
use crate::WINDOW_HEIGHT;
use crate::enemy::Behaviour::{FlyL2R, FlyR2L};

const SHOOT_INTERVAL: f64 = 10.0;

pub struct Enemy {
    health: u8,
    speed: f64,
    position_x: f64,
    position_y: f64,
    behaviour: Behaviour,
    shoot_timer: f64,
    sprite_id: SpriteID,
}

enum Behaviour {
    FlyL2R,
    FlyR2L,
}

impl Enemy {
    pub fn new(speed: f64) -> Enemy {
        let position_x = rand::thread_rng().gen_range(-WALK_AREA_MAX_X..WALK_AREA_MAX_X);
        let behaviour = match position_x < 0.0 {
            true => FlyL2R,
            false => FlyR2L
        };

        Enemy {
            health: 5,
            speed,
            position_x,
            position_y: WINDOW_HEIGHT as f64 + 50.0,
            behaviour,
            shoot_timer: SHOOT_INTERVAL,
            sprite_id: SpriteID(1)
        }
    }

    pub fn damage(&mut self) {
        if self.health >= 1 {
            self.health -= 1;
        }
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

impl Entity for Enemy {
    fn on_update(&mut self, _game_status: &mut GameStatus, renderer: &mut Renderer, sprites: &mut SpriteBank, _input: &Input, delta_time: f64, game_speed: f64) {
        self.shoot_timer -= game_speed * delta_time;
        self.position_y -= self.speed * game_speed * delta_time;

        match self.behaviour {
            FlyL2R => {
                self.position_x += self.speed / 2.0 * game_speed * delta_time;
            }
            FlyR2L => {
                self.position_x -= self.speed / 2.0 * game_speed * delta_time;
            }
        };

        let sprite = sprites.get_sprite(&self.sprite_id).unwrap();

        let x_offset = (sprite.get_width() / 2) as i64;
        let y_offset = (sprite.get_height() / 2) as i64;

        renderer.sprite(sprite, self.position_x as i64 - x_offset, self.position_y as i64 - y_offset, false);

        renderer.line(
            self.position_x as i64 - 2,
            self.position_y as i64 + 8,
            self.position_x as i64 - 2,
            self.position_y as i64 + 8 + rand::thread_rng().gen_range(0..5),
            FlaskColor::Yellow as u8
        );
        renderer.line(
            self.position_x as i64 - 1,
            self.position_y as i64 + 8,
            self.position_x as i64 - 1,
            self.position_y as i64 + 8 + rand::thread_rng().gen_range(2..7),
            FlaskColor::White as u8
        );
        renderer.line(
            self.position_x as i64,
            self.position_y as i64 + 8,
            self.position_x as i64,
            self.position_y as i64 + 8 + rand::thread_rng().gen_range(0..5),
            FlaskColor::Yellow as u8
        );
    }

    fn x(&self) -> i64 {
        self.position_x as i64
    }

    fn y(&self) -> i64 {
        self.position_y as i64
    }

    fn collider_radius(&self) -> i64 {
        5
    }
}

