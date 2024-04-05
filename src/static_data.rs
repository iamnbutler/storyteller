use crate::{new_id, Choice, NarrativeSegment, Scenario};

pub fn intro_scenario() -> Scenario {
    let first_segment = NarrativeSegment {
    id: new_id(),
    narrative: "You wake up in a damp cave, with no memory of how you got there. You are wearing nothing, and see nothing around you in the faint light but a bent iron spoon and a small wooden plate.".to_string(),
    choices: vec![
        Choice {
            id: new_id(),
            text: "Pick up the spoon".to_string(),
            consequence: "You pick up the spoon and add it to your inventory".to_string(),
            next_segment_id: None,
        },
        Choice {
            id: new_id(),
            text: "Pick up the plate".to_string(),
            consequence: "You pick up the plate and add it to your inventory".to_string(),
            next_segment_id: None,

        },
        Choice {
            id: new_id(),
            text: "Ignore the objects and head towards the light".to_string(),
            consequence: "".to_string(),
            next_segment_id: None,
        },
    ]};

    let second_segment = NarrativeSegment {
    id: new_id(),
    narrative: "You walk towards the light, and find yourself in a small clearing. You see a small stream, and a path leading into the forest. You hear a loud screeching noise overhead.".to_string(),
    choices: vec![],
    };

    Scenario {
        id: new_id(),
        title: "Introduction".to_string(),
        description:
            "You wake up alone in a dark cave, not knowing what this world has in store for you."
                .to_string(),
        segments: vec![first_segment, second_segment],
    }
}
