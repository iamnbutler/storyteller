#!/usr/bin/env python3
import os
from openai import OpenAI
from dotenv import load_dotenv
from join import join_markdown_files

load_dotenv()

ASSISTANT_INSTRUCTION = """
You are an expert at creating D&D characters. Generate a balanced 4-person party for a campaign starting in the city of Baldur's Gate.
Each character should be level 5 or below and have a simple yet interesting backstory that explains why they are in Baldur's Gate.
Include a mix of classes and races that would be commonly found in a diverse city setting. Ensure the characters have potential for
interesting interactions but also personal goals that could align with a group's objectives.
"""

PROMPT_DIR = "src/prompts"
CONTEXT_DIR = "src/context"

client = OpenAI(
    api_key=os.getenv("OPENAI_API_KEY")
)

try:
    # Join the party handbook to the input context
    join_markdown_files(['background', 'party-handbook'], 'output/party-handbook-context.md')

    with open('output/party-handbook-context.md', 'r') as file:
        context = file.read()

    response = client.chat.completions.create(
        model="gpt-4",
        messages=[
            {"role": "system", "content": ASSISTANT_INSTRUCTION + "\n" + context},
            {"role": "user", "content": "Please create the party now."},
        ]
    )

    assistant_reply = response.choices[0].message.content

    # Save to the specified output file
    with open("output/party.md", "w") as md_file:
        md_file.write(assistant_reply)

    print("Successfully generated a party and saved to output/party.md")

except Exception as e:
    print("Couldn't create output. There was an error:")
    print(e)
