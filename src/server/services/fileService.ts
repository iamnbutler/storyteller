import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import { promisify } from "util";
import { exec } from "child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const STORY_PATH = path.join(__dirname, "storyteller");

async function saveFileToStoryPath(relativeFilePath: string, content: string): Promise<void> {
  const fullPath = path.join(STORY_PATH, relativeFilePath);
  await fs.writeFile(fullPath, content);
}

async function saveMarkdownToStoryPath(
  relativeFilePath: string,
  markdownContent: string,
): Promise<void> {
  const fullPath = path.join(STORY_PATH, relativeFilePath);
  await fs.writeFile(fullPath, markdownContent);
}

async function readFileFromStoryPath(relativeFilePath: string): Promise<string> {
  const fullPath = path.join(STORY_PATH, relativeFilePath);
  const content = await fs.readFile(fullPath, "utf8");
  return content;
}

async function renameFileInStoryPath(
  oldRelativeFilePath: string,
  newRelativeFilePath: string,
): Promise<void> {
  const oldFullPath = path.join(STORY_PATH, oldRelativeFilePath);
  const newFullPath = path.join(STORY_PATH, newRelativeFilePath);
  await fs.rename(oldFullPath, newFullPath);
}

async function deleteFileFromStoryPath(relativeFilePath: string): Promise<void> {
  const fullPath = path.join(STORY_PATH, relativeFilePath);
  await fs.unlink(fullPath);
}

const execAsync = promisify(exec);

async function joinMarkdownFiles(filePaths: string[], outputFilePath: string): Promise<void> {
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

export {
  saveFileToStoryPath,
  saveMarkdownToStoryPath,
  readFileFromStoryPath,
  renameFileInStoryPath,
  deleteFileFromStoryPath,
  joinMarkdownFiles,
};
