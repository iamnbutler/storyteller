import { Choice, useGameState} from "./store";

const CHOICE_PROMPT = `Generate a set of choices based off of the following stpry context.

Return a group of choices in this format:

~~~json
[
  {
    "label": "Report the information to the city guard",
    "consequence": "You decide to report the information to the city guard. After your shift you head to the closest Watch outpust. [...] You contemplate your fate, arrested for being an accomplice to the crime, despite doing what you thought was right and reporting what you heard. [...]"
  },
  {
    "label": "Investigate the warehouse yourself",
    "consequence": "You decide to investigate the warehouse yourself. After your shift, you head to the warehouse and find it to be a trap. [...]"
  },
  {
    "label": "Hint that you might want to join their next heist",
    "consequence": "You decide to hint that you might want to join their next heist. After your shift, you approach the group and express your interest in joining them. [...]"
  },
  {
    "label": "Ignore the information and continue with your day",
    "consequence": "You decide to ignore the information and continue with your day. After your shift, you head home and try to forget what you heard. [...]"
  }
]
~~~

Don't actually include the "[...]" in the "consequence" field. That's just a placeholder for the rest of the story. Write a sentence for each consequence.

DON'T include less than 2 choices, or more than 5 choices
DO occasionally add twists in the consequence, like the example of being arrested when reporting the crime yourself in the example above.

Here is the story context:`

const choicePrompt = (context: string) => `${CHOICE_PROMPT}\n\n${context}`;

export const fetchChoicesData = async (context: string): Promise<Choice[]> => {
  try {
    const response = await fetch('http://localhost:3000/openai/completion', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ content: choicePrompt(context) }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    console.log('Fetched data:', data);
    const strippedData = data.completion.replace(/```json\n/, '').replace(/\n```/, '');
    console.log('Stripped data:', strippedData);
    const parsedData = JSON.parse(strippedData);
    console.log('Parsed data:', parsedData);

    if (!Array.isArray(parsedData)) {
      throw new Error('Expected an array of choices, got: ${parsedData}');
    }

    const {createChoice} = useGameState.getState();
    const choices = parsedData.map((item: {label: string, consequence: string}) => {
      const choice = createChoice(item.label, item.consequence);
      return choice;
    });
    return choices

  } catch (error) {
    console.error('Error fetching choices:', error);
    throw error;
  }
};

export const fetchChoices = async (context: string) => {
  try {
    const data = await fetchChoicesData(context);
    return data;
  } catch (error) {
    console.error('Error fetching choices:', error);
  }
};
