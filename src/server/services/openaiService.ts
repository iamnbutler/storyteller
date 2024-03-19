import { saveMarkdown, readMarkdown } from "./fileService";

import dotenv from "dotenv";
dotenv.config({ path: "../../.env" });

const openai_key = process.env.OPENAI_API_KEY;

if (!openai_key) {
  throw new Error("OpenAI API key not found");
}

import OpenAI from "openai";

const openai = new OpenAI({ apiKey: openai_key });

const MODEL = "gpt-4-0125-preview";

export async function completion(content: string): Promise<string> {
  try {
    const completionResult = await openai.chat.completions.create({
      messages: [
        { role: "system", content: "You are a helpful assistant designed to output JSON." },
        { role: "user", content },
      ],
      model: MODEL,
      response_format: { type: "json_object" },
    });

    const messageContent = completionResult.choices[0].message.content;

    return messageContent || "No response";
  } catch (error) {
    throw error;
  }
}
