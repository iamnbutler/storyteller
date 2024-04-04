use async_recursion::async_recursion;
use async_std::task;
use dialoguer::Select;
use save::{build_save_path, get_save_path};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use valico::json_schema;
mod save;

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct GameContext {
    segments: HashMap<String, StorySegment>,
    choices: HashMap<String, Choice>,
}

impl GameContext {
    fn new() -> Self {
        Self {
            segments: HashMap::new(),
            choices: HashMap::new(),
        }
    }

    pub fn build_json_schema() -> std::io::Result<()> {
        // Generate schema for GameContext
        let schema = schema_for!(GameContext);
        let schema_json =
            serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

        // Example usage of your get_save_path function
        let path = get_save_path().unwrap_or_else(|_| PathBuf::from("."));
        let schema_file_path = path.join("game_context_schema.json");

        let mut file = File::create(schema_file_path)?;
        writeln!(file, "{}", schema_json)?;

        Ok(())
    }
}

fn load_and_validate_game_context() -> Result<(), Box<dyn std::error::Error>> {
    // Load the schema and the JSON instance (same as your code)
    let schema_path = build_save_path("game_context_schema.json")?;
    let schema_json = load_json(schema_path)?;
    let game_context_path = build_save_path("game_context.json")?;
    let game_context_json = load_json(game_context_path)?;

    // Create a new scope and compile the schema
    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(schema_json, false)?;

    // Validate the instance against the compiled schema
    let state = schema.validate(&game_context_json);

    // Check if there were any errors
    if state.is_valid() {
        println!("The game context is valid according to the schema.");
    } else {
        println!("Validation failed:");
        for error in &state.errors {
            println!("- {}", error.get_title());
        }
        for suberror in &state.missing {
            println!("- Missing: {}", suberror);
        }
        return Err("Validation failed".into());
    }

    Ok(())
}

fn load_json(file_path: PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: Value = serde_json::from_str(&contents)?;
    Ok(json)
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
struct StorySegment {
    id: String,
    narrative: String,
    choices: Vec<Choice>,
}

impl StorySegment {
    fn new(narrative: &str, choices: Vec<Choice>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            narrative: narrative.to_string(),
            choices,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
struct Choice {
    id: String,
    text: String,
    consequence: String,
    next_segment: Option<Box<StorySegment>>,
}

impl Choice {
    fn new(text: &str, consequence: &str, next_segment: Option<StorySegment>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            consequence: consequence.to_string(),
            next_segment: next_segment.map(Box::new),
        }
    }
}

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let game_context = GameContext::build_json_schema();
    if let Err(e) = game_context {
        eprintln!("Error generating JSON schema: {}", e);
    }

    if let Err(e) = load_and_validate_game_context() {
        eprintln!("Error: {}", e);
    }

    start_game().await;
}

async fn start_game() {
    let cx = Arc::new(RwLock::new(GameContext::new()));
    println!("Starting the game...");

    let intro_segment = StorySegment::new(
        "You stand before the ancient ruins...",
        vec![
            Choice::new(
                "Enter the ruins",
                "The air is cool and damp...",
                Some(
                    StorySegment::new(
                        "As you proceed, the corridor splits in two directions.",
                        vec![
                            Choice::new("Go left", "You find an ancient relic.", None),
                            Choice::new("Go right", "A sudden drop awaits. It's a dead end, but you manage to climb out safely.", None),
                        ]
                    )
                )
            ),
            Choice::new("Leave", "You decide to leave...", None),
        ],
    );

    play_segment(cx, intro_segment).await;
}

async fn play_segment(cx: Arc<RwLock<GameContext>>, segment: StorySegment) {
    cx.write()
        .unwrap()
        .segments
        .insert(segment.id.clone(), segment.clone());

    println!("\n{}", segment.narrative);
    if !segment.choices.is_empty() {
        show_choices(cx, segment.choices).await;
    } else {
        println!("This path has come to an end.");
    }
}

#[async_recursion]
async fn show_choices(cx: Arc<RwLock<GameContext>>, choices: Vec<Choice>) {
    let selections = choices
        .iter()
        .map(|choice| choice.text.as_str())
        .collect::<Vec<_>>();

    let selection = Select::new()
        .with_prompt("What do you do?")
        .default(0)
        .items(&selections)
        .interact()
        .unwrap();

    cx.write()
        .unwrap()
        .choices
        .insert(choices[selection].id.clone(), choices[selection].clone());

    println!("\n{}", choices[selection].consequence);

    if let Some(next_segment) = choices[selection].next_segment.as_ref() {
        play_segment(cx, (**next_segment).clone()).await;
    }
}
