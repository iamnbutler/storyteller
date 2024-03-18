import { create } from "zustand";
import { nanoid } from "nanoid";

export type Choice = {
  id: string;
  /* The label of the choice in the ui */
  label: string;
  /* Additional context for the choice, like party info or story context */
  context: string;
  /* If this specific choice has been chosen */
  chosen: boolean;
  /* if this choice is the current choice */
  current: boolean;
};

export const createChoice = (label: string, context: string): Choice => ({
  id: nanoid(),
  label,
  context,
  chosen: false,
  current: false,
});

export type StorySegment = {
  id: string;
  story: string;
  choices?: Choice[];
};

export const createStorySegment = (text: string): StorySegment => {
  return {
    id: nanoid(),
    story: text,
  };
};

type GameState = {
  story: StorySegment[];
  setStory: (story: StorySegment[]) => void;
  createStorySegment: (text: string) => StorySegment;
  choices: Choice[];
  createChoice: (label: string, context: string) => Choice;
  setChoices: (choices: Choice[]) => void;
  currentChoices: Choice[];
  setCurrentChoices: (choices: Choice[]) => void;
};

export const useGameState = create<GameState>()((set) => ({
  story: [],
  choices: [],
  createChoice,
  setStory: (story: StorySegment[]) => set({ story }),
  setChoices: (choices: Choice[]) => set({ choices }),
  createStorySegment,
  currentChoices: [],
  setCurrentChoices: (currentChoices: Choice[]) => set({ currentChoices }),
}));
