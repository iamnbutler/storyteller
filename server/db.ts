import Database from "better-sqlite3";
const db = new Database("story.db", { verbose: console.log });

const initDB = (): void => {
  db.exec(`
    CREATE TABLE IF NOT EXISTS segments (
      id TEXT PRIMARY KEY,
      text TEXT,
      selected_choice_id TEXT,
      FOREIGN KEY (selected_choice_id) REFERENCES choices(id)
    );

    CREATE TABLE IF NOT EXISTS choices (
      id TEXT PRIMARY KEY,
      segment_id TEXT,
      label TEXT,
      FOREIGN KEY (segment_id) REFERENCES segments(id)
    );
  `);
};

const insertSegment = (id: string, text: string): void => {
  const stmt = db.prepare("INSERT INTO segments (id, text) VALUES (?, ?)");
  stmt.run(id, text);
};

const insertChoice = (id: string, segmentId: string, label: string): void => {
  const stmt = db.prepare("INSERT INTO choices (id, segment_id, label) VALUES (?, ?, ?)");
  stmt.run(id, segmentId, label);
};

const getSegmentById = (id: string): StorySegment => {
  const stmt = db.prepare("SELECT * FROM segments WHERE id = ?");
  return stmt.get(id);
};

const getChoicesBySegmentId = (segmentId: string): StoryChoice[] => {
  const stmt = db.prepare("SELECT * FROM choices WHERE segment_id = ?");
  return stmt.all(segmentId);
};

export { initDB, insertSegment, insertChoice, getSegmentById, getChoicesBySegmentId };
