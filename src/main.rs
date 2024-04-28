#![allow(unused)]

use app::window_options;
use dialoguer::Select;
use gpui::*;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use valico::json_schema;

mod app;
mod ui;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
}

impl Player {
    fn new(name: String) -> Self {
        Self { name }
    }
}

pub type ScenarioID = Uuid;

#[derive(Debug)]
pub struct Scenario {
    id: ScenarioID,
    narrative: Vec<String>,
}

#[derive(Debug)]
pub struct GameData {
    player: Player,
    scenario: HashMap<ScenarioID, Scenario>,
}

#[derive(Debug)]
pub struct Game {
    data: Model<GameData>,
}

impl Game {
    fn new(cx: &mut AppContext) -> Self {
        let data = cx.new_model(|cx| GameData {
            player: Player::new("Nate".to_string()),
            scenario: HashMap::new(),
        });

        Self { data }
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(window_options(540, 960, cx), |cx| {
            let game = Game::new(cx);

            cx.new_view(|_cx| ui::GameWindow { game })
        });
    });
}
