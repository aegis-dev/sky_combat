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

use flask::scene::Scene;
use flask::renderer::Renderer;
use flask::game_status::GameStatus;
use flask::input::{Input, State, Key};
use flask::font::Font;
use flask::palette::FlaskColor;

use crate::game::Game;

pub struct GameOver {
    score: u128,
    font: Font,
}

impl GameOver {
    pub fn new(score: u128) -> GameOver {
        GameOver {
            score,
            font: Font::load_3x5().unwrap()
        }
    }
}

impl Scene for GameOver {
    fn on_start(&mut self, renderer: &mut Renderer) {
        renderer.set_background_color(FlaskColor::White as u8).unwrap();
    }

    fn on_update(&mut self, game_status: &mut GameStatus, renderer: &mut Renderer, input: &Input, _delta_time: f64) -> Option<Box<dyn Scene>> {
        let game_over = String::from("GAME OVER");
        let score_text = String::from("SCORE:");
        let score = String::from(format!("{}", self.score));
        renderer.text(&game_over, &self.font, -(4 * 5), 0, FlaskColor::Red as u8);
        renderer.text(&score_text, &self.font, -(4 * 3), -10, FlaskColor::Red as u8);
        renderer.text(&score, &self.font, -(4 * (score.len() as i64 / 2)) , -20, FlaskColor::Yellow as u8);

        if input.get_key_state(Key::Return) == State::Down {
            return Some(Box::new(Game::new()))
        }

        if input.get_key_state(Key::Escape) == State::Down {
            game_status.quit();
        }

        None
    }

    fn on_destroy(&mut self) {

    }
}