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

use crate::entity::Entity;

pub struct Projectile {
    speed: f64,
    angle: f64,
    position_x: f64,
    position_y: f64,
    color: u8,
}

impl Projectile {
    pub fn new(speed: f64, angle: f64, position_x: i64, position_y: i64, color: u8) -> Projectile {
        Projectile {
            speed,
            angle,
            position_x: position_x as f64,
            position_y: position_y as f64,
            color
        }
    }
}

impl Entity for Projectile {
    fn on_update(&mut self, _game_status: &mut GameStatus, renderer: &mut Renderer, _sprites: &mut SpriteBank, _input: &Input, delta_time: f64, game_speed: f64) {
        let radians = self.angle * (std::f64::consts::PI / 180.0);

        let dir_x = radians.cos();
        let dir_y = radians.sin();

        self.position_x += dir_x * self.speed * game_speed * delta_time;
        self.position_y += dir_y * self.speed * game_speed * delta_time;

        renderer.circle(self.position_x as i64, self.position_y as i64, 2, self.color);
        renderer.circle_filled(self.position_x as i64, self.position_y as i64, 1, FlaskColor::White as u8);
    }

    fn x(&self) -> i64 {
        self.position_x as i64
    }

    fn y(&self) -> i64 {
        self.position_y as i64
    }

    fn collider_radius(&self) -> i64 {
        2
    }
}