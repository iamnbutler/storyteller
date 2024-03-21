import { nanoid } from "nanoid";

/** A story segment is a piece of the story including story text,
 * choices that the player can make, and the id of the choice
 * that the player made if they have made a choice.
 */
export type StorySegment = {
  /** The unique id of the segment */
  id: string;
  /** The text of the segment */
  text: string;
  /** The choices that the player can make */
  choices: Array<{ id: string; label: string }>;
  /** The id of the segment that comes after this one */
  selected_choice_id?: string;
};

/** Creates a new story segment from the given text context */
export const newSegment = (text: string): StorySegment => ({
  id: nanoid(),
  text,
  choices: [],
});

/** Finds a given segment by its id */
export const getSegmentById = (segments: Array<StorySegment>, segment_id: string) => {
  const segment = segments.find((s) => s.id === segment_id);

  if (!segment) {
    throw new Error(`Segment with id ${segment_id} not found`);
  }

  return segment;
};

/** Adds a choice to the given segment */
export const addChoiceToSegment = (segment: StorySegment, label: string): StorySegment => ({
  ...segment,
  choices: [
    ...segment.choices,
    {
      id: nanoid(),
      label,
    },
  ],
});

export const getChoiceById = (segment: StorySegment, choice_id: string) => {
  const choice = segment.choices.find((c) => c.id === choice_id);

  if (!choice) {
    throw new Error(`Choice with id ${choice_id} not found`);
  }

  return choice;
};

/** Selects a choice for the given segment */
export const selectChoice = (segment: StorySegment, choice_id: string): StorySegment => ({
  ...segment,
  selected_choice_id: choice_id,
});

/** Creates a new story segment based on the choice that the player made */
export const nextSegment = (previous_segment: StorySegment, choice_id: string): StorySegment => {
  const choice = getChoiceById(previous_segment, choice_id);

  const next_segment = newSegment(`TODO: create segment from choice: ${choice.label}`);

  return next_segment;
};

export const createOptimisticTextFromChoice = async (
  segment: StorySegment,
  choice_id: string,
): Promise<string> => {
  const choice = getChoiceById(segment, choice_id);

  const optomistic_text = `TOOD: Generate text from ${choice.label}`;

  return optomistic_text;
};
