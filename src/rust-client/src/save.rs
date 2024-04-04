#![allow(unused)]

use crate::{Choice, GameContext, StorySegment};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub fn get_save_path() -> Result<PathBuf, io::Error> {
    let project_dir = env::current_dir()?;
    let save_path = project_dir.join("data");
    Ok(save_path)
}

pub fn build_save_path(file_name: &str) -> Result<PathBuf, io::Error> {
    let save_path = get_save_path()?;
    Ok(save_path.join(file_name))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SaveData {
    segments: Vec<StorySegment>,
    choices: Vec<Choice>,
}

impl SaveData {
    /// Saves the current [GameContext]
    pub fn save(cx: &GameContext) -> Self {
        Self {
            segments: cx.segments.values().cloned().collect(),
            choices: cx.choices.values().cloned().collect(),
        }
    }

    pub fn quicksave(cx: &GameContext) -> Result<(), io::Error> {
        let save_data = Self::save(cx);
        let save_path = get_save_path()?.join(format!(
            "quicksave_{}.json",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));
        let file = File::create(save_path)?;
        serde_json::to_writer(file, &save_data)?;
        Ok(())
    }

    /// Loads save data from a given JSON string
    pub fn load_from_json(json_data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_data)
    }

    /// Loads save data from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, LoadError> {
        // Attempt to open the file
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Err(LoadError::FileNotFound(file_path.as_ref().to_path_buf()))
            }
            Err(_) => return Err(LoadError::CannotReadFile(file_path.as_ref().to_path_buf())),
        };

        // Read the contents of the file
        let mut json_data = String::new();
        if let Err(_) = file.read_to_string(&mut json_data) {
            return Err(LoadError::CannotReadFile(file_path.as_ref().to_path_buf()));
        }

        // Attempt to deserialize the JSON data
        serde_json::from_str(&json_data).map_err(LoadError::Serde)
    }
}

#[derive(Debug)]
pub enum LoadError {
    FileNotFound(std::path::PathBuf),
    CannotReadFile(std::path::PathBuf),
    Serde(SerdeError),
}

impl From<LoadError> for Box<dyn std::error::Error> {
    fn from(e: LoadError) -> Box<dyn std::error::Error> {
        match e {
            LoadError::Serde(e) => Box::new(e),
            _ => Box::new(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
        }
    }
}
