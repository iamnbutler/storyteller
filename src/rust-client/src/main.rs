use async_recursion::async_recursion;
use async_std::task;
use dialoguer::Select;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone)]
struct GameContext {
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
}

#[derive(Clone)]
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

#[derive(Clone)]
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
