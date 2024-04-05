import { nanoid } from "nanoid";

type Choice = {
  id: string;
  label: string;
};

type ChoiceSet = {
  id: string;
  choices: Choice[];
  chosen_id?: string;
};

/** A story segment is a piece of the story including story text,
 * choices that the player can make, and the id of the choice
 * that the player made if they have made a choice.
 */
type Segment = {
  /** The unique id of the segment */
  id: string;
  /** The text of the segment */
  text: string;
  /** The choices that the player can make */
  choices: ChoiceSet[];
  /** The id of the segment that comes after this one */
  selected_choice_id?: string;
};

type Story = {
  segments: Segment[];
  newSegment: (text: string) => Segment;
  getCurrentSegment: () => Segment;
  setCurrentSegment: (segment: Segment) => void;
};

const story: Story = {
  segments: [],
  newSegment: (text: string) => {
    const newSegment: Segment = {
      id: nanoid(),
      text,
      choices: [],
    };
    story.segments.push(newSegment);
    return newSegment;
  },
  getCurrentSegment: () => {
    return story.segments[story.segments.length - 1];
  },
  setCurrentSegment: (segment: Segment) => {
    story.segments.push(segment);
  },
};
