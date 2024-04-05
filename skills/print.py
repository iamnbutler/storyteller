#!/usr/bin/env python3
from openai import OpenAI
import os
from dotenv import load_dotenv

load_dotenv()

ASSISTANT_INSTRUCTION = "You are a narrator hired to give life and voice to rpg worlds and stories. Narrate the given scenarios and make the story come alive. Take small pauses before clearly narrating each decision after."

PROMPT_DIR = "src/prompts"

client = OpenAI(
    api_key=os.getenv("OPENAI_API_KEY")
)

try:
    prompt_name = input("Please enter the name of the prompt: ")
    if not prompt_name.endswith(".md"):
        prompt_name += ".md"
    prompt_path = os.path.join(PROMPT_DIR, prompt_name)
    with open(prompt_path, 'r') as file:
        user_query = file.read()

    response = client.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=[
            {"role": "system", "content": ASSISTANT_INSTRUCTION},
            {"role": "user", "content": user_query},
        ]
    )

    assistant_reply = response.choices[0].message.content

    with open("output.md", "w") as md_file:
        md_file.write(assistant_reply)

    print("Successfully got response from OpenAI and saved to output.md")

except Exception as e:
    print("Couldn't create output. There was an error:")
    print(e)
