import express, { Request, Response } from "express";
import dotenv from "dotenv";
import * as openaiService from "./services/openaiService";

dotenv.config();

const app = express();
const port: number = parseInt(process.env.PORT || "3000");

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

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
