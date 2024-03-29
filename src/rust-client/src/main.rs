use dialoguer::Select;

fn main() {
    let options = ["Start Game", "Exit"];
    let selection = Select::new()
        .with_prompt("Please choose an action")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => start_game(),
        1 => println!("Exiting game."),
        _ => unreachable!(),
    }
}

fn start_game() {
    println!("Starting the game...");

    let intro_segment = StorySegment::new(
        "You find yourself standing at a crossroads in a dense forest. To your left, the path leads deeper into the darkness of the woods. To your right, a gentle light beckons.",
        vec![
            Choice::new("Go left into the darkness", "The path is treacherous, but you press on."),
            Choice::new("Go right towards the light", "The light grows brighter as you approach."),
        ],
    );

    play_segment(intro_segment);
}

fn play_segment(segment: StorySegment) {
    println!("\n{}", segment.narrative);
    show_choices(segment.choices);
}

fn show_choices(choices: Vec<Choice>) {
    if choices.is_empty() {
        println!("There are no choices to make here.");
        return;
    }

    let selection = Select::new()
        .with_prompt("What do you do?")
        .default(0)
        .items(
            &choices
                .iter()
                .map(|choice| choice.text.as_str())
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();

    println!("{}", choices[selection].result);
}

struct Choice {
    text: String,
    result: String,
}

impl Choice {
    fn new(text: &str, result: &str) -> Choice {
        Choice {
            text: text.to_string(),
            result: result.to_string(),
        }
    }
}

struct StorySegment {
    narrative: String,
    choices: Vec<Choice>,
}

impl StorySegment {
    fn new(narrative: &str, choices: Vec<Choice>) -> StorySegment {
        StorySegment {
            narrative: narrative.to_string(),
            choices,
        }
    }
}
