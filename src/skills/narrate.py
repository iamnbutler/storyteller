#!/usr/bin/env python3
from openai import OpenAI
import os
from dotenv import load_dotenv

# load all environment variables
load_dotenv()

ASSISTANT_INSTRUCTION = "You are Layla, a personal assistant. Answer the user's questions helpfully and make their day easier."

client = OpenAI(
    api_key=os.getenv("OPENAI_API_KEY")
)

print("Starting the app...")

try:
    # Get user's query
    user_query = input("Please enter your query: ")

    response = client.chat.completions.create(
        model="gpt-4",  # replace this if gpt-4 is not available
        messages=[
            {"role": "system", "content": ASSISTANT_INSTRUCTION},
            {"role": "user", "content": user_query},
        ]
    )

    assistant_reply = response.choices[0].message.content

    # Create voice audio of the assistant's reply
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
