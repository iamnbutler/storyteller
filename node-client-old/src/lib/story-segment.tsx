import { nanoid } from "nanoid";
import * as db from "./db.js";

export type StoryChoice = {
  id: string;
  label: string;
};

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
  choices: StoryChoice[];
  /** The id of the segment that comes after this one */
  selected_choice_id?: string;
};

/** Creates a new story segment from the given text context */
const newSegment = (text: string): StorySegment => {
  const id = nanoid();
  db.insertSegment(id, text);
  return { id, text, choices: [] };
};

/** Finds a given segment by its id */
const getSegmentById = (segmentId: string): StorySegment => {
  const segment = db.getSegmentById(segmentId);
  if (!segment) throw new Error(`Segment with id ${segmentId} not found`);
  const choices = db.getChoicesBySegmentId(segmentId);
  return { ...segment, choices };
};

/** Adds a choice to the given segment */
const addChoiceToSegment = (segmentId: string, label: string): StorySegment => {
  const choiceId = nanoid();
  db.insertChoice(choiceId, segmentId, label);
  const updatedSegment = getSegmentById(segmentId);
  return updatedSegment;
};

const getChoiceById = (segment: StorySegment, choice_id: string): StoryChoice => {
  const choice = segment.choices.find((c) => c.id === choice_id);
  if (!choice) {
    throw new Error(`Choice with id ${choice_id} not found`);
  }
  return choice;
};

/** Selects a choice for the given segment */
const selectChoice = (segment: StorySegment, choice_id: string): StorySegment => {
  return {
    ...segment,
    selected_choice_id: choice_id,
  };
};

/** Creates a new story segment based on the choice that the player made */
const nextSegment = (previous_segment: StorySegment, choice_id: string): StorySegment => {
  const choice = getChoiceById(previous_segment, choice_id);
  const next_segment = newSegment(`Next segment based on choice: ${choice.label}`);

  return next_segment;
};

const createOptimisticTextFromChoice = (segment: StorySegment, choice_id: string): string => {
  const choice = getChoiceById(segment, choice_id);
  const optimistic_text = `Optimistically generated text from choice: ${choice.label}`;
  return optimistic_text;
};

export {
  newSegment,
  getSegmentById,
  addChoiceToSegment,
  selectChoice,
  nextSegment,
  createOptimisticTextFromChoice,
};
