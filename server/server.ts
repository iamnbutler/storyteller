import express, { Request, Response } from "express";
import dotenv from "dotenv";
import * as openaiService from "./services/openaiService";
import cors from "cors";
import * as db from "./db";
import * as storyService from "./services/storyService";

dotenv.config();

const app = express();
const port: number = parseInt(process.env.PORT || "3000");

app.use(cors());
app.use(express.json());

app.post("/openai/completion", async (req: Request, res: Response) => {
  try {
    const content: string = req.body.content;
    const completionResult = await openaiService.completion(content);
    res.json({ completion: completionResult });
  } catch (error) {
    res.status(500).json({ error: "An error occurred while fetching the completion." });
  }
});

// Endpoint for creating a new story segment
app.post("/api/story_segments", async (req: Request, res: Response) => {
  try {
    const { text } = req.body;
    const newSegment = await storyService.newSegment(text);
    res.status(201).json(newSegment);
  } catch (error) {
    console.error("Failed to create a story segment:", error);
    res.status(500).json({ error: "An error occurred while creating the story segment." });
  }
});

// Endpoint for fetching choices for a given story segment
app.get("/api/choices/:segmentId", async (req: Request, res: Response) => {
  try {
    const { segmentId } = req.params;
    const choices = await storyService.getChoicesBySegmentId(segmentId);
    res.json(choices);
  } catch (error) {
    console.error("Failed to fetch choices for the story segment:", error);
    res.status(404).json({ error: "Choices not found for the given segment." });
  }
});

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
