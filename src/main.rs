#![allow(unused)]

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

mod input;
mod ui;

pub fn window_options(app_width: i32, app_height: i32, cx: &AppContext) -> WindowOptions {
    let display_id_maybe = cx.displays().last().map(|d| d.id());
    let bounds_maybe = cx.displays().last().map(|d| d.bounds());
    let bounds = bounds_maybe.unwrap_or(Bounds {
        origin: Point::new(DevicePixels::from(0), DevicePixels::from(0)),
        size: Size {
            width: DevicePixels::from(1920),
            height: DevicePixels::from(1080),
        },
    });

    let mut options = WindowOptions::default();
    let center = bounds.center();

    options.focus = true;
    options.display_id = display_id_maybe;
    let width = DevicePixels::from(app_width);
    let height = DevicePixels::from(app_height);
    let x: DevicePixels = center.x - width / 2;
    let y: DevicePixels = center.y - height / 2;

    let bounds: Bounds<DevicePixels> = Bounds::new(Point { x, y }, Size { width, height });
    options.bounds = Some(bounds);
    options.titlebar = Some(TitlebarOptions::default());
    options.is_movable = true;
    options.kind = WindowKind::PopUp;
    options
}

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
        cx.open_window(window_options(540, 720, cx), |cx| {
            let game = Game::new(cx);

            cx.new_view(|cx| ui::GameWindow::new(cx, game))
        });
    });
}
