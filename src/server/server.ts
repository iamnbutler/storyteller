import express, { Request, Response } from "express";
import dotenv from "dotenv";
import * as fileService from "./services/fileService";
import * as openaiService from "./services/openaiService";

dotenv.config();

const app = express();
const port: number = parseInt(process.env.PORT || "3000");

app.use(express.json());

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
