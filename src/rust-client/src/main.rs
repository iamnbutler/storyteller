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
    let start_segment = StorySegment::new(
        "As the dawn breaks, you find yourself at the entrance of the ancient ruins.",
        vec![
            Choice::new("Enter the ruins", "You step cautiously into the darkness.", Some(
                StorySegment::new(
                    "The air is cool and damp. As your eyes adjust, you notice a faint light emanating from deeper within.",
                    vec![
                        Choice::new("Follow the light", "You make your way towards the light.", None),
                        Choice::new("Explore the surroundings", "You decide to explore the nearby rooms first.", None),
                    ],
                )
            )),
            Choice::new("Circle around", "You decide to walk around the ruins' perimeter first.", None),
        ],
    );

    play_segment(start_segment);
}

fn play_segment(segment: StorySegment) {
    println!("\n{}", segment.narrative);
    if !segment.choices.is_empty() {
        show_choices(segment.choices);
    } else {
        println!("This path has come to an end.");
    }
}

fn show_choices(choices: Vec<Choice>) {
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

    // Provide immediate feedback
    println!("\n{}", choices[selection].consequence);

    // If the choice leads to another segment, play it
    if let Some(next_segment) = choices[selection].next_segment.as_ref() {
        play_segment((**next_segment).clone()); // Cloning here for simplicity; a more complex game might avoid this
    }
}

#[derive(Debug, Clone)]
struct Choice {
    text: String,
    consequence: String, // Immediate consequence or description upon choosing
    next_segment: Option<Box<StorySegment>>, // Using Option and Box to allow for optional and heap-allocated StorySegment
}

impl Choice {
    fn new(text: &str, consequence: &str, next_segment: Option<StorySegment>) -> Choice {
        Choice {
            text: text.to_string(),
            consequence: consequence.to_string(),
            next_segment: next_segment.map(Box::new), // Convert StorySegment into a Boxed Option
        }
    }
}

#[derive(Debug, Clone)]
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
