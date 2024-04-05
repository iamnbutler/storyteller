#![allow(unused)]

use dialoguer::Select;
use gpui::*;
use save::{build_save_path, get_save_path, SaveData};
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

mod save;

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct GameContext {
    segments: HashMap<String, StorySegment>,
    choices: HashMap<String, Choice>,
    save_data: Option<SaveData>,
    character: Option<Character>,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            segments: HashMap::new(),
            choices: HashMap::new(),
            save_data: None,
            character: None,
        }
    }

    pub fn add_segment(cx: Arc<Mutex<GameContext>>, new_segment: StorySegment) {
        let mut game_ctx = cx.lock().unwrap(); // Obtains a lock and panics on error
        game_ctx
            .segments
            .insert(new_segment.id.clone(), new_segment);
    }

    pub fn get_segment(&self, id: &str) -> Option<StorySegment> {
        self.segments.get(id).cloned()
    }

    pub fn get_segments(&self) -> Vec<StorySegment> {
        self.segments.values().cloned().collect()
    }

    pub fn add_choice(cx: Arc<Mutex<GameContext>>, new_choice: Choice) {
        let mut game_ctx = cx.lock().unwrap(); // Same approach as above
        game_ctx.choices.insert(new_choice.id.clone(), new_choice);
    }

    pub fn get_choice(&self, id: &str) -> Option<Choice> {
        self.choices.get(id).cloned()
    }

    pub fn add_character(&mut self, character: Character) {
        self.character = Some(character);
    }

    pub fn get_character(&self) -> Option<Character> {
        self.character.clone()
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

fn load_and_validate_game_context(
    file_path: PathBuf,
) -> Result<GameContext, Box<dyn std::error::Error>> {
    let schema_path = build_save_path("game_context_schema.json")?;
    let schema_json = load_json(schema_path)?;

    // Instead of loading generic JSON, directly load GameContext
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let game_context: GameContext = serde_json::from_str(&contents)?;

    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(schema_json, false)?;

    // Validate GameContext as serde_json::Value for schema compliance
    let game_context_json: Value = serde_json::from_str(&contents)?;
    let state = schema.validate(&game_context_json);

    if !state.is_valid() {
        // Detailed error printing omitted for brevity
        return Err("Validation failed with detailed errors".into());
    }

    Ok(game_context)
}

fn load_json(file_path: PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: Value = serde_json::from_str(&contents)?;
    Ok(json)
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Character {
    surname: String,
    family_name: Option<String>,
    nickname: Option<String>,
}

impl Character {
    /// Create a new [Character]
    pub fn new(surname: &str, family_name: Option<&str>, nickname: Option<&str>) -> Self {
        Self {
            surname: surname.to_string(),
            family_name: family_name.map(|s| s.to_string()),
            nickname: nickname.map(|s| s.to_string()),
        }
    }

    /// Get the full name of the character.
    ///
    /// If a `family_name` is provided, it will be included in the full name.
    ///
    /// If a `nickname` is provided, it will added between the `surname` and `family_name`.
    pub fn full_name(&self) -> String {
        let mut full_name = self.surname.clone();
        if let Some(nickname) = &self.nickname {
            full_name.push_str(" \"");
            full_name.push_str(nickname);
            full_name.push_str("\" ");
        }
        if let Some(family_name) = &self.family_name {
            full_name.push_str(family_name);
        }
        full_name
    }

    /// Get the display name of the character.
    ///
    /// If a `nickname` is provided, it will be used as the display name,
    /// otherwise the `surname` will be used.
    pub fn display_name(&self) -> String {
        if let Some(nickname) = &self.nickname {
            nickname.clone()
        } else {
            self.surname.clone()
        }
    }

    /// Get a formatted character sheet.
    pub fn character_sheet(&self) {
        let sheet = format!(
            "Character Sheet\n\nSurname: {}\nFamily Name: {}\nNickname: {}\n",
            self.surname,
            self.family_name.as_deref().unwrap_or("N/A"),
            self.nickname.as_deref().unwrap_or("N/A")
        );

        println!("{}", sheet);
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct StorySegment {
    id: String,
    narrative: String,
    choices: Vec<String>,
}

impl StorySegment {
    fn new(narrative: &str, choice_ids: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            narrative: narrative.to_string(),
            choices: choice_ids,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_narrative(&self) -> String {
        self.narrative.clone()
    }

    pub fn get_choice_ids(&self) -> Vec<String> {
        self.choices.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Choice {
    id: String,
    text: String,
    consequence: String,
    next_segment: Option<String>,
}

impl Choice {
    fn new(text: &str, consequence: &str, next_segment: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            consequence: consequence.to_string(),
            next_segment,
        }
    }
}

fn start_game(game_context: GameContext) {
    println!("Starting the game...");

    let cx = Arc::new(Mutex::new(game_context));

    character_creation(cx.clone());

    let first_segment_id = {
        let game_context = cx.lock().unwrap();
        game_context.segments.values().next().unwrap().id.clone()
    };

    play_segment(cx.clone(), first_segment_id);
}

fn character_creation(cx: Arc<Mutex<GameContext>>) {
    println!("Create your character");

    let surname = "Jason";
    let family_name = Some("Asano");
    let nickname = Some("Starlight Rider");

    let character = Character::new(&surname, family_name.as_deref(), nickname.as_deref());

    cx.lock().unwrap().add_character(character.clone());

    println!("Character created: {}", character.full_name());
}

fn play_segment(cx: Arc<Mutex<GameContext>>, segment_id: String) {
    let segment = {
        let cx_read = cx.lock().unwrap();
        cx_read
            .get_segment(&segment_id)
            .expect("Segment ID not found")
    };

    println!("\n{}", segment.narrative);
    if !segment.choices.is_empty() {
        show_choices(cx, segment.choices);
    } else {
        println!("This path has come to an end.");
    }
}

fn show_choices(cx: Arc<Mutex<GameContext>>, choice_ids: Vec<String>) {
    let mut selections = {
        let cx_read = cx.lock().unwrap();
        choice_ids
            .iter()
            .map(|id| {
                cx_read
                    .choices
                    .get(id)
                    .expect("Choice ID not found")
                    .text
                    .clone()
            })
            .collect::<Vec<String>>()
    };

    // Add "More options" to the list of choices
    selections.push("More options".to_string());

    let selection = Select::new()
        .with_prompt("What do you do?")
        .default(0)
        .items(&selections)
        .interact()
        .unwrap();

    // Check if the user selected "More options"
    if selection == selections.len() - 1 {
        show_more_options(cx);
    } else {
        let selected_choice_id = &choice_ids[selection];
        let selected_choice = {
            let cx_read = cx.lock().unwrap();
            cx_read
                .get_choice(selected_choice_id)
                .expect("Choice ID not found")
        };

        println!("\n{}", selected_choice.consequence);

        if let Some(next_segment) = selected_choice.next_segment {
            play_segment(cx, next_segment);
        }
    }
}

fn show_more_options(cx: Arc<Mutex<GameContext>>) {
    let cx_read = cx.lock().unwrap();

    let character = cx_read.character.clone();

    let current_segment_id = cx_read
        .segments
        .keys()
        .last()
        .expect("No segments saved")
        .clone();

    let more_options = vec!["Back to Choices", "Show character sheet", "Quit Game"];

    let selection = Select::new()
        .with_prompt("Select an option")
        .default(0)
        .items(&more_options)
        .interact()
        .unwrap();

    match selection {
        0 => play_segment(cx.clone(), current_segment_id),
        1 => {
            if let Some(character) = character {
                character.character_sheet();
                show_more_options(cx.clone());
            } else {
                println!("No character created yet.")
            }
        }
        2 => {
            println!("Quitting game...");
            std::process::exit(0);
        }
        _ => panic!("Unexpected option"),
    }
}

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("{}", &self.text))
    }
}

fn main() {
    let context_path = if let Ok(path) = build_save_path("game_context.json") {
        path
    } else {
        eprintln!("Error building save path for game context");
        return;
    };

    let game_context = match load_and_validate_game_context(context_path) {
        Ok(gc) => gc,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    GameContext::build_json_schema().expect("Error generating JSON schema");

    start_game(game_context.clone());

    let text = game_context
        .segments
        .values()
        .next()
        .unwrap()
        .get_narrative();

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld { text: text.into() })
        });
    });
}
