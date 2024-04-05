#![allow(unused)]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::static_data::intro_scenario;

struct Action {
    /// Starts a new game
    start_game: Box<dyn Fn(&mut GameContext) -> GameResult>,
    /// Begins a new scenario
    start_scenario: Box<dyn Fn(&mut GameContext, &str) -> GameResult>,
    /// Completes a scenario
    complete_scenario: Box<dyn Fn(&mut GameContext, &str) -> GameResult>,
    /// Links from one scenario to another via a choice
    link_scenario: Box<dyn Fn(&mut GameContext, &str, &str, &str) -> GameResult>,
    /// Makes a choice in a scenario
    make_choice: Box<dyn Fn(&mut GameContext, &str) -> GameResult>,
}

#[derive(Debug)]
enum GameError {
    GenericError(String),
    ScenarioNotFound(String),
    ChoiceNotFound(String),
    Other(String),
}

type GameResult = Result<(), GameError>;

pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Scenario {
    id: String,
    title: String,
    description: String,
    segments: Vec<NarrativeSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct NarrativeSegment {
    id: String,
    narrative: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Choice {
    id: String,
    text: String,
    consequence: String,
    next_segment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct PlayerProgress {
    character: Character,
    scenarios: PlayerScenarios,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct PlayerScenarios {
    current_scenario_id: Option<String>,
    current_segment_id: String,
    active_scenarios: Vec<String>,
    scenario_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Character {
    id: String,
    name: String,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Attribute {
    name: String,
    value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameContext {
    version: String,
    player_progress: Option<PlayerProgress>,
    scenarios: Vec<Scenario>,
    start_scenario_id: String,
}

fn start_game(cx: &mut GameContext) -> GameResult {
    println!("Storyteller - Version: {}\n", cx.version);

    cx.start_scenario_id = intro_scenario().id;
    Ok(())
}

fn start_screen() {
    println!("Welcome to Storyteller!\n");
}

fn start_scenario(cx: &mut GameContext, scenario_id: &str) -> GameResult {
    if cx.player_progress.is_none() {
        return Err(GameError::GenericError(
            "Game has not been initialized properly. No player progress found.".to_string(),
        ));
    }

    let player_progress = cx
        .player_progress
        .as_mut()
        .expect("Player progress not initialized");

    match cx.scenarios.iter().find(|s| s.id == scenario_id).cloned() {
        Some(scenario) => {
            println!("Starting scenario: {}", scenario.title);

            player_progress.scenarios.current_scenario_id = Some(scenario_id.to_string());
            player_progress.scenarios.current_segment_id = scenario
                .segments
                .first()
                .map_or_else(|| "".to_string(), |segment| segment.id.clone());

            if !player_progress
                .scenarios
                .active_scenarios
                .contains(&scenario_id.to_string())
            {
                player_progress
                    .scenarios
                    .active_scenarios
                    .push(scenario_id.to_string());
            }

            Ok(())
        }
        None => Err(GameError::ScenarioNotFound(format!(
            "Scenario ID '{}' not found",
            scenario_id
        ))),
    }
}

fn main() {
    let actions = Action {
        start_game: Box::new(start_game),
        start_scenario: Box::new(|_, _| {
            Err(GameError::GenericError("Not Implemented".to_string()))
        }),
        complete_scenario: Box::new(|_, _| {
            Err(GameError::GenericError("Not Implemented".to_string()))
        }),
        link_scenario: Box::new(|_, _, _, _| {
            Err(GameError::GenericError("Not Implemented".to_string()))
        }),
        make_choice: Box::new(|_, _| Err(GameError::GenericError("Not Implemented".to_string()))),
    };

    let mut game = GameContext {
        version: env!("CARGO_PKG_VERSION").to_string(),
        player_progress: None,
        scenarios: vec![],
        start_scenario_id: "".to_string(),
    };

    match (actions.start_game)(&mut game) {
        Ok(_) => println!("Starting a new game."),
        Err(e) => println!("Error starting game: {:?}", e),
    }
}
