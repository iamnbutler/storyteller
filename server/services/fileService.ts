import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import { promisify } from "util";
import { exec } from "child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const STORY_PATH = path.join(__dirname, "storyteller");

export async function saveMarkdown(
  relativeFilePath: string,
  markdownContent: string,
): Promise<void> {
  const fullPath = path.join(STORY_PATH, relativeFilePath);
  await fs.writeFile(fullPath, markdownContent);
}

export async function readMarkdown(relativeFilePath: string): Promise<string> {
  const fullPath = path.join(STORY_PATH, relativeFilePath);
  const content = await fs.readFile(fullPath, "utf8");
  return content;
}

const execAsync = promisify(exec);

export async function joinMarkdown(filePaths: string[], outputFilePath: string): Promise<void> {
  const storyDir = path.join(STORY_PATH, "context");
  const fullOutputPath = path.join(STORY_PATH, outputFilePath);
  const filesToJoin = filePaths.map((fileName) => path.join(storyDir, `${fileName}.md`));
  const command = `cat ${filesToJoin.join(" ")} > ${fullOutputPath}`;

  try {
    await execAsync(command);
    console.log(`Markdown files have been joined into ${outputFilePath}`);
  } catch (error) {
    console.error(`Error joining markdown files: ${error}`);
  }
}
