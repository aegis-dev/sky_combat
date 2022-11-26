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

use flask::rand::Rand;
use flask::scene::Scene;
use flask::renderer::Renderer;
use flask::game_status::GameStatus;
use flask::input::{Input, Key, State};
use flask::font::Font;
use flask::palette::FlaskColor;
use flask::sprite::{SpriteBank, Sprite};

use crate::player::Player;
use crate::entity::Entity;
use crate::enemy::Enemy;
use crate::projectile::Projectile;
use crate::game_over::GameOver;
use crate::explosion::Explosion;
use crate::cloud::Cloud;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const WALK_AREA_MAX_X: f64 = (WINDOW_WIDTH / 2) as f64;
pub const WALK_AREA_MAX_Y: f64 = WINDOW_HEIGHT as f64;
pub const SPAWN_INTERVAL: f64 = 10.0;

pub struct Game {
    font: Font,
    score: u128,
    game_speed: f64,
    player: Player,
    spawn_timer: f64,
    score_timer: f64,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    player_projectiles: Vec<Projectile>,
    explosions : Vec<Explosion>,
    clouds: Vec<Cloud>,
    sprite_bank: SpriteBank,
    rng: Rand,
}

impl Game {
    pub fn new() -> Game {
        Game {
            font: Font::load_3x5().unwrap(),
            score: 0,
            game_speed: 10.0,
            player: Player::new(),
            spawn_timer: SPAWN_INTERVAL * 2.0,
            score_timer: 0.0,
            enemies: vec![],
            projectiles: vec![],
            player_projectiles: vec![],
            explosions: vec![],
            clouds: vec![],
            sprite_bank: SpriteBank::new(),
            rng: Rand::new()
        }
    }
}

impl Scene for Game {
    fn on_start(&mut self, renderer: &mut Renderer) {
        self.sprite_bank.add_sprite(Sprite::from_indexed_8bit_png(include_bytes!("../assets/player.png")).unwrap());
        self.sprite_bank.add_sprite(Sprite::from_indexed_8bit_png(include_bytes!("../assets/enemy.png")).unwrap());

        for _ in 0..30 {
            self.clouds.push(Cloud::new(&mut self.rng));
        }

        renderer.set_background_color(FlaskColor::Teal as u8).unwrap();
        renderer.set_camera_y(renderer.get_window_size().1 / 2);
    }

    fn on_update(&mut self, game_status: &mut GameStatus, renderer: &mut Renderer, input: &Input, delta_time: f64) -> Option<Box<dyn Scene>> {
        if input.get_key_state(Key::Escape) == State::Down {
            game_status.quit();
            return None;
        }

        self.score_timer += delta_time * self.game_speed;
        if self.score_timer >= 1.0 {
            self.score += 1;
            self.score_timer -= 1.0;
        }

        self.spawn_timer -= delta_time * self.game_speed;
        if self.spawn_timer < 0.0 {
            self.spawn_timer = SPAWN_INTERVAL;
            self.enemies.push(Enemy::new(5.0, self.rng.next_u64()))
        }


        for cloud in &mut self.clouds {
            cloud.on_update(game_status, renderer, &mut self.sprite_bank, input, delta_time, self.game_speed);
        }

        let mut enemies_to_remove = vec![];
        for i in 0..self.enemies.len() {
            let enemy = &mut self.enemies[i];
            enemy.on_update(game_status, renderer, &mut self.sprite_bank, input, delta_time, self.game_speed);

            if enemy.can_shoot() {
                let angle = enemy.angle(&self.player);
                if enemy.y() < 250 && enemy.y() > 30 {
                    self.projectiles.push(Projectile::new(10.0, angle, enemy.x(), enemy.y(), FlaskColor::Red as u8));
                    enemy.reset_shoot_interval();
                }
            }

            if enemy.y() < -20 || enemy.x() > WALK_AREA_MAX_X as i64 + 20 || enemy.x() < -WALK_AREA_MAX_X as i64 - 20 {
                enemies_to_remove.push(i);
            }
        }

        let mut projectiles_to_remove = vec![];
        for i in 0..self.projectiles.len() {
            let projectile = &mut self.projectiles[i];
            projectile.on_update(game_status, renderer, &mut self.sprite_bank, input, delta_time, self.game_speed);

            if projectile.intersects(&self.player) {
                self.player.damage();
                projectiles_to_remove.push(i);
            }

            if projectile.y() < -20 || projectile.distance(&self.player) > 300.0 {
                projectiles_to_remove.push(i);
            }
        }

        if !self.player.alive() {
            return Some(Box::new(GameOver::new(self.score)));
        }

        self.player.on_update(game_status, renderer, &mut self.sprite_bank, input, delta_time, self.game_speed);

        if self.player.can_shoot() {
            self.player_projectiles.push(Projectile::new(30.0, 90.0, self.player.x() - 10, self.player.y(), FlaskColor::Yellow as u8));
            self.player_projectiles.push(Projectile::new(30.0, 90.0, self.player.x() + 10, self.player.y(), FlaskColor::Yellow as u8));
            self.player.reset_shoot_interval();
        }

        let mut player_projectiles_to_remove = vec![];
        for i in 0..self.player_projectiles.len() {
            let projectile = &mut self.player_projectiles[i];
            projectile.on_update(game_status, renderer, &mut self.sprite_bank, input, delta_time, self.game_speed);

            if projectile.distance(&self.player) > 400.0 {
                player_projectiles_to_remove.push(i);
                continue;
            }

            for y in 0..self.enemies.len() {
                let enemy = &mut self.enemies[y];
                if projectile.intersects(enemy) {
                    enemy.damage();
                    player_projectiles_to_remove.push(i);
                    if !enemy.alive() {
                        self.score += 100;
                        enemies_to_remove.push(y);
                        self.explosions.push(Explosion::new(enemy.x(), enemy.y()));
                        break;
                    }
                }
            }
        }

        let mut explosions_to_remove = vec![];
        for i in 0..self.explosions.len() {
            let explosion = &mut self.explosions[i];
            explosion.on_update(game_status, renderer, &mut self.sprite_bank, input, delta_time, self.game_speed);

            if !explosion.alive() {
                explosions_to_remove.push(i);
            }
        }

        let (window_w, window_h) = renderer.get_window_size();

        let score_text = String::from(format!("SCORE: {}", self.score));
        renderer.text(&score_text, &self.font, -(window_w / 2) + 5, window_h - 10, FlaskColor::Purple as u8);

        let lives_text = String::from(format!("LIVES: {}", self.player.health()));
        renderer.text(&lives_text, &self.font, -(window_w / 2) + 5, window_h - 20, FlaskColor::Purple as u8);

        // Remove things
        enemies_to_remove.sort();
        enemies_to_remove.dedup();
        for i in enemies_to_remove.iter().rev() {
            self.enemies.remove(*i);
        }
        projectiles_to_remove.sort();
        for i in projectiles_to_remove.iter().rev() {
            self.projectiles.remove(*i);
        }
        player_projectiles_to_remove.sort();
        for i in player_projectiles_to_remove.iter().rev() {
            self.player_projectiles.remove(*i);
        }
        explosions_to_remove.sort();
        for i in explosions_to_remove.iter().rev() {
            self.explosions.remove(*i);
        }

        None
    }

    fn on_destroy(&mut self) {

    }
}
