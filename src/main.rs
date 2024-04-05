#![allow(unused)]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

struct Action {
    /// Starts a new game
    start_game: Box<dyn Fn(&mut Game) -> GameResult>,
    /// Begins a new scenario
    start_scenario: Box<dyn Fn(&mut Game, &str) -> GameResult>,
    /// Completes a scenario
    complete_scenario: Box<dyn Fn(&mut Game, &str) -> GameResult>,
    /// Links from one scenario to another via a choice
    link_scenario: Box<dyn Fn(&mut Game, &str, &str, &str) -> GameResult>,
    /// Makes a choice in a scenario
    make_choice: Box<dyn Fn(&mut Game, &str) -> GameResult>,
}

#[derive(Debug)]
enum GameError {
    GenericError(String),
    ScenarioNotFound(String),
    ChoiceNotFound(String),
    Other(String),
}

type GameResult = Result<(), GameError>;

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
struct Game {
    version: String,
    player_progress: Option<PlayerProgress>,
    scenarios: Vec<Scenario>,
    start_scenario_id: String,
}

fn start_game(game: &mut Game) -> GameResult {
    println!("Storyteller - Version: {}\n", game.version);

    game.start_scenario_id = "initial_scenario_id".to_owned();
    Ok(())
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

    let mut game = Game {
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
