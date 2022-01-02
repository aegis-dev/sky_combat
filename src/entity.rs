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
use flask::sprite::SpriteBank;

pub trait Entity {
    fn on_update(&mut self, game_status: &mut GameStatus, renderer: &mut Renderer, sprites: &mut SpriteBank, input: &Input, delta_time: f64, game_speed : f64);

    fn x(&self) -> i64;

    fn y(&self) -> i64;

    fn collider_radius(&self) -> i64;

    fn distance(&self, other: &dyn Entity) -> f64 {
        (((self.x() - other.x()).pow(2) + (self.y() - other.y()).pow(2)) as f64).sqrt()
    }

    fn angle(&self, other: &dyn Entity) -> f64 {
        let delta_x = (other.x() - self.x()) as f64;
        let delta_y = (other.y() - self.y()) as f64;
        let radians = delta_y.atan2(delta_x);
        radians * 180.0 / std::f64::consts::PI
    }

    fn intersects(&self, other: &dyn Entity) -> bool {
        (self.distance(other) - self.collider_radius() as f64 - other.collider_radius() as f64) < 0.0
    }
}
