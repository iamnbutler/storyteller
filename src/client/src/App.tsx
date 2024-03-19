import { useEffect } from "react";
import { theme } from "./lib/theme";
import { useGameState } from "./lib/store";
import { ChoicesView } from "./lib/choice";

const STORY_CONTEXT = `Baldur's Gate, a city of opportunity and danger, stands as a bustling metropolis on the Sword Coast—a place where ambition and intrigue mingle with commerce and trade. It's a city of stark contrasts, where the wealthy patriarchy resides within the Upper City while the Lower City teems with working folk, gangs, and the destitute. The recent events of the "Iron Crisis," with its turmoil and strife, have passed—but not without leaving scars and tales that still echo through the cobbled streets and tavern whispers.`;

function App() {
  const {
    story,
    choices,
    currentChoices,
    setStory,
    setChoices,
    setCurrentChoices,
    createChoice,
    createChoiceGroup,
    createStorySegment,
  } = useGameState();

  const initial_choices = createChoiceGroup([
    createChoice("Report the information to the city guard."),
    createChoice("Investigate the warehouse yourself."),
    createChoice("Hint that you want to join the next heist."),
    createChoice("Ignore the conversation."),
  ]);

  useEffect(() => {
    setStory([createStorySegment(STORY_CONTEXT)]);
    setChoices(initial_choices);
    setCurrentChoices(initial_choices[0].group_id);
  }, []);

  return (
    <>
      <div className="h-screen w-screen overflow-hidden">
        <div
          style={{
            background: theme.background,
            color: theme.text.primary,
            fontSize: "12px",
            lineHeight: "16px",
          }}
          className="mx-auto mt-64 h-[248px] w-[440px]"
        >
          {story.map((segment, index) => (
            <div key={index}>
              <p>{segment.story}</p>
              <hr className="border-white/10" />
              {segment.choices.length > 0 && (
                <ol>
                  <ChoicesView choices={currentChoices} />
                </ol>
              )}
            </div>
          ))}
        </div>
      </div>
      <div className="absolute right-0 top-0 flex w-64 flex-col gap-4 overflow-hidden text-xs">
        <pre>
          Choices
          {JSON.stringify(choices, null, 2)}
        </pre>
        <pre>
          Current Choices
          {JSON.stringify(currentChoices, null, 2)}
        </pre>
      </div>
    </>
  );
}

export default App;
