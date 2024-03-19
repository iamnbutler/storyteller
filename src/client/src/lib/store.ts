import { create } from "zustand";
import { nanoid } from "nanoid";

export type Choice = {
  group_id: string;
  id: string;
  /* The label of the choice in the ui */
  label: string;
  /* If this specific choice has been chosen */
  chosen: boolean;
  /* if this choice is the current choice */
  current: boolean;
};

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
  createChoice: (label: string) => Choice;
  createChoiceGroup: (choices: Choice[]) => Choice[];
  setChoices: (choices: Choice[]) => void;
  currentChoices: Choice[];
  setCurrentChoices: (group_id: string) => void;
  setChosenChoice: (id: string) => void;
};

export const useGameState = create<GameState>()((set) => ({
  story: [],
  choices: [],
  createChoice: (label: string): Choice => ({
    group_id: "",
    id: nanoid(),
    label,
    chosen: false,
    current: false,
  }),
  createChoiceGroup: (choices: Choice[]) => {
    const new_group_id = nanoid();

    return choices.map((choice) => {
      return {
        ...choice,
        group_id: new_group_id,
      };
    });
  },
  setStory: (story: StorySegment[]) => set({ story }),
  setChoices: (choices: Choice[]) => set({ choices }),
  createStorySegment,
  currentChoices: [],
  setCurrentChoices: (group_id: string) => {
    set((state) => {
      const currentChoices = state.choices.filter((choice) => choice.group_id === group_id);
      return { currentChoices };
    });
  },
  setChosenChoice: (id: string) => {
    set((state) => {
      const chosenGroupId = state.choices.find((choice) => choice.id === id)?.group_id;
      const choices = state.choices.map((choice) => {
        if (choice.group_id === chosenGroupId) {
          return { ...choice, chosen: choice.id === id };
        }
        return choice;
      });
      return { choices };
    });
  },
}));
