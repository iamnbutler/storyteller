import { create } from "zustand";
import { nanoid } from "nanoid";

export type Choice = {
  id: string;
  label: string;
  chosen: boolean;
};

export type StorySegment = {
  id: string;
  story: string;
  choices: Choice[];
};

type GameState = {
  story: StorySegment[];
  setStory: (story: StorySegment[]) => void;
  createStorySegment: (text: string) => StorySegment;
  addChoicesToStorySegment: (choices: Choice[], segment_id: string) => void;
  createChoice: (label: string) => Choice;
  setChosenChoice: (segmentId: string, choiceId: string) => void;
};

export const useGameState = create<GameState>()((set) => ({
  story: [],
  setStory: (story: StorySegment[]) => set({ story }),
  createStorySegment: (text: string) => {
    const newSegment: StorySegment = {
      id: nanoid(),
      story: text,
      choices: [],
    };
    set((state) => ({
      story: [...state.story, newSegment],
    }));
    return newSegment;
  },
  addChoicesToStorySegment: (choices: Choice[], segment_id: string) => {
    set((state) => ({
      story: state.story.map((segment) => {
        if (segment.id === segment_id) {
          return { ...segment, choices: [...segment.choices, ...choices] };
        }
        return segment;
      }),
    }));
  },
  createChoice: (label: string): Choice => ({
    id: nanoid(),
    label,
    chosen: false,
  }),
  setChosenChoice: (segmentId: string, choiceId: string) => {
    set((state) => ({
      story: state.story.map((segment) => {
        if (segment.id === segmentId) {
          const updatedChoices = segment.choices.map((choice) => ({
            ...choice,
            chosen: choice.id === choiceId,
          }));
          return { ...segment, choices: updatedChoices };
        }
        return segment;
      }),
    }));
  },
}));
