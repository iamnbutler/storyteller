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
    createStorySegment,
  } = useGameState();

  const initial_choices = [
    createChoice("Report the information to the city guard.", STORY_CONTEXT),
    createChoice("Investigate the warehouse yourself.", STORY_CONTEXT),
    createChoice("Hint that you want to join the next heist.", STORY_CONTEXT),
    createChoice("Ignore the conversation.", STORY_CONTEXT),
  ];

  useEffect(() => {
    setStory([createStorySegment(STORY_CONTEXT)]);
    setChoices(initial_choices);
    setCurrentChoices(initial_choices);
  }, []);

  return (
    <>
      <div className="h-screen w-screen overflow-hidden">
        <div
          style={{
            background: theme.background,
            color: theme.text.primary,
            fontSize: "11px",
            lineHeight: "16px",
          }}
          className="mx-auto mt-64 h-[248px] w-[440px]"
        >
          {story.map((segment, index) => (
            <div key={index}>
              <p>{segment.story}</p>
              <hr />
              <ol>
                <ChoicesView choices={currentChoices} current={true} />
              </ol>
              <hr />
              <ol>
                <ChoicesView choices={currentChoices} current={false} />
              </ol>
            </div>
          ))}
        </div>
      </div>
    </>
  );
}

export default App;
