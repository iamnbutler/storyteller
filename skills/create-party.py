#!/usr/bin/env python3
import os
from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

ASSISTANT_INSTRUCTION = """
You are an expert at creating D&D characters. Generate a 4-person party for a campaign starting in the city of Baldur's Gate.
Each character should be level 5 or below and have a simple yet interesting backstory that explains why they are in Baldur's Gate.
Ensure the characters have potential for interesting interactions but also personal goals that could align with a group's objectives.
Avoid using any existing characters from the Baldur's Gate series or other D&D campaigns, or iconic first or last names from other fantasy series.
Ensure a complete character sheet is created for each character. Follow the guidelines and rules provided in the party handbook.
"""

EXTEND_BACKGROUND_PROMPT = """
You are a skilled storyteller. Complete the second half of the campaign's background story using the party as reference. Avoid introducing the characters directly one at a time, but instead, weave their introductions into the shared first quest the players will embark on and how their paths crossed to get there.
"""

BACKGROUND_PATH = "src/context/background.md"
PARTY_HANDBOOK_PATH = "src/context/party-handbook.md"

client = OpenAI(
    api_key=os.getenv("OPENAI_API_KEY")
)

try:
    # Read the party handbook
    with open(PARTY_HANDBOOK_PATH, "r") as file:
        party_handbook_content = file.read()

    # Combine handbook content with the assistant instruction
    combined_instruction = party_handbook_content + "\n" + ASSISTANT_INSTRUCTION

    # Generate Party
    response = client.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=[
            {"role": "system", "content": combined_instruction},
            {"role": "user", "content": "Please create the party now."},
        ]
    )

    assistant_reply = response.choices[0].message.content

    # Save the generated party info to party.md
    with open("output/party.md", "w") as md_file:
        md_file.write(assistant_reply)

    print("Successfully generated a party and saved to output/party.md")

    # Read the background before the extension
    with open(BACKGROUND_PATH, "r") as bg_file:
        background_content = bg_file.read()

    # Generate the extended background and include the merged content
    extended_response = client.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=[
            {"role": "system", "content": EXTEND_BACKGROUND_PROMPT + "\n" + background_content + "\n" + assistant_reply},
            {"role": "user", "content": "Please extend the background now."},
        ]
    )

    extended_intro = extended_response.choices[0].message.content

    # Save the extended introduction to intro.md
    with open("output/intro.md", "w") as intro_file:
        intro_file.write(background_content + "\n" + extended_intro)

    print("Successfully extended the background and saved to output/intro.md")

except Exception as e:
    print("Couldn't create output. There was an error:")
    print(e)
