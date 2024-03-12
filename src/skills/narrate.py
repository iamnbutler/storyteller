#!/usr/bin/env python3
from openai import OpenAI
import os
from dotenv import load_dotenv

load_dotenv()

ASSISTANT_INSTRUCTION = "You are a narrator hired to give life and voice to rpg worlds and stories. Narrate the given scenarios and make the story come alive. Take small pauses before clearly narrating each decision after."

PROMPT_DIR = "../prompts"

client = OpenAI(
    api_key=os.getenv("OPENAI_API_KEY")
)

try:
    user_query = input("Please enter your query: ")

    response = client.chat.completions.create(
        model="gpt-4",
        messages=[
            {"role": "system", "content": ASSISTANT_INSTRUCTION},
            {"role": "user", "content": user_query},
        ]
    )

    assistant_reply = response.choices[0].message.content

    voice_response = client.audio.speech.create(
        model="tts-1",
        voice="nova",
        input=assistant_reply,
    )

    voice_response.stream_to_file("output.mp3")

    print("Successfully got response from OpenAI and saved to output.mp3")

except Exception as e:
    print("Couldn't create output. There was an error:")
    print(e)
