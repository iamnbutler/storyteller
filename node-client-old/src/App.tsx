import { useEffect } from "react";
import { useGameState } from "./lib/store";
import { theme } from "./lib/theme";
import { ChoicesView } from "./ui/choices";
import { fetchChoices } from "./lib/choice";

const STORY_CONTEXT = `Baldur's Gate, a city of opportunity and danger, stands as a bustling metropolis on the Sword Coast—a place where ambition and intrigue mingle with commerce and trade. It's a city of stark contrasts, where the wealthy patriarchy resides within the Upper City while the Lower City teems with working folk, gangs, and the destitute. The recent events of the "Iron Crisis," with its turmoil and strife, have passed—but not without leaving scars and tales that still echo through the cobbled streets and tavern whispers.`;

function App() {
  const {
    story,
    setStory,
    createStorySegment,
    addChoicesToStorySegment,
    setCurrentChoicesLoaded,
    currentChoicesLoaded,
  } = useGameState();

  useEffect(() => {
    let canceled = false;
    const newStorySegment = createStorySegment(STORY_CONTEXT);
    setStory([newStorySegment]);

    async function fetch() {
      const choices = await fetchChoices(STORY_CONTEXT);

      if (!canceled && choices && choices.length > 0) {
        addChoicesToStorySegment(choices, newStorySegment.id);
        setCurrentChoicesLoaded(true);
      } else if (!canceled) {
        throw new Error("No choices received");
      }
    }

    fetch();

    return () => {
      canceled = true;
    };
  }, [createStorySegment, setStory, addChoicesToStorySegment, setCurrentChoicesLoaded]);

  useEffect(() => {
    async function fetch() {
      const choices = await fetchChoices(STORY_CONTEXT);

      if (choices && choices.length > 0) {
        addChoicesToStorySegment(choices, story[0].id);
        setCurrentChoicesLoaded(true);
      } else {
        throw new Error("No choices recieved");
      }
    }

    fetch();
  }, []);

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
          className="mx-auto mt-64 h-[248px] w-[440px] pt-4"
        >
          {currentSegment ? (
            <div key={currentSegment.id}>
              <p className="px-4">{currentSegment.story}</p>
              {currentChoicesLoaded && currentSegment.choices.length > 0 && (
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
      <div className="absolute right-0 top-0 flex w-64 flex-col gap-4 overflow-hidden text-xs opacity-10 hover:opacity-100">
        <pre>{JSON.stringify(story, null, 2)}</pre>
      </div>
    </>
  );
}

export default App;
