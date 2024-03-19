import { useEffect } from "react";
import { useGameState } from "./lib/store";
import { ChoicesView } from "./lib/choice";
import { theme } from "./lib/theme";

const STORY_CONTEXT = `Baldur's Gate, a city of opportunity and danger, stands as a bustling metropolis on the Sword Coast—a place where ambition and intrigue mingle with commerce and trade. It's a city of stark contrasts, where the wealthy patriarchy resides within the Upper City while the Lower City teems with working folk, gangs, and the destitute. The recent events of the "Iron Crisis," with its turmoil and strife, have passed—but not without leaving scars and tales that still echo through the cobbled streets and tavern whispers.`;

function App() {
  const { story, setStory, createStorySegment, addChoicesToStorySegment, createChoice } =
    useGameState();

  useEffect(() => {
    const newStorySegment = createStorySegment(STORY_CONTEXT);
    setStory([newStorySegment]);

    addChoicesToStorySegment(
      [
        createChoice("Report the information to the city guard."),
        createChoice("Investigate the warehouse yourself."),
        createChoice("Hint that you want to join the next heist."),
        createChoice("Ignore the conversation."),
      ],
      newStorySegment.id,
    );
  }, [createChoice, createStorySegment, setStory, addChoicesToStorySegment]);

  const currentSegment = story[0];

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
          {currentSegment ? (
            <div key={currentSegment.id}>
              <p>{currentSegment.story}</p>
              <hr className="border-white/10" />
              {currentSegment.choices.length > 0 && (
                <ol>
                  <ChoicesView segment={currentSegment} current={true} />
                </ol>
              )}
            </div>
          ) : (
            <p>Loading story...</p>
          )}
        </div>
      </div>
      <div className="absolute right-0 top-0 flex w-64 flex-col gap-4 overflow-hidden text-xs">
        <pre>{JSON.stringify(story, null, 2)}</pre>
      </div>
    </>
  );
}

export default App;
