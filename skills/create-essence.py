import os
from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

ASSISTANT_INSTRUCTION = """
You are an expert at storytelling and creating game items and mechanics. Based on the given context, follow the user's instructions and return the data they ask for.
"""

ESSENCE_HANDBOOK = "context/worldbuilding/essence-handbook.md"
ESSENCE_EXAMPLES = "context/worldbuilding/essence-examples.md"

client = OpenAI(
    api_key=os.getenv("OPENAI_API_KEY")
)

try:
    # Read the essence handbook
    with open(ESSENCE_HANDBOOK, "r") as file:
        essence_handbook_content = file.read()

    # Read the essence examples
    with open(ESSENCE_EXAMPLES, "r") as file:
        essence_examples_content = file.read()

    # Combine handbook content with the assistant instruction
    combined_instruction = essence_handbook_content + "\n" + essence_examples_content + "\n" + ASSISTANT_INSTRUCTION

    # Generate Essence
    response = client.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=[
            {"role": "system", "content": combined_instruction},
            {"role": "user", "content": "Please create essence-related content now."},
        ]
    )

    assistant_reply = response.choices[0].message.content

    # Save the generated essence info to essence.md
    with open("output/create_an_essence.md", "w") as md_file:
        md_file.write(assistant_reply)

    print("Successfully generated essence-related content and saved to output/create_an_essence.md")

except Exception as e:
    print("Couldn't create output. There was an error:")
    print(e)
