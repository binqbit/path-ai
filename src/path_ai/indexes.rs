use std::{fs, vec};

use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::{get_exec_path, hashing, indexed_tree, DirTreeNode, Config};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Indexes {
    pub time: i64,
    pub hash: String,
    pub tokens: usize,
    pub dirs: Vec<DirTreeNode>,
}



impl Indexes {
    pub fn load() -> Self {
        let path = get_exec_path().join("data.json");
        std::fs::read_to_string(path)
            .map(|content| serde_json::from_str(&content).unwrap())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = get_exec_path().join("data.json");
        let content = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, content).expect("Failed to save data");
    }

    pub fn scan(&mut self, config: &Config) {
        println!("Scanning indexes...");

        let path = get_exec_path().join("indexes.txt");
        let content = std::fs::read_to_string(path).expect("Indexes file error");
        let hash: String = hashing(&content);
        let indexes = content
                .split("\n")
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>();
        self.dirs = indexed_tree(indexes);
        self.time = Utc::now().timestamp();
        self.hash = hash;
        self.tokens = tiktoken::count_text(config.get_gpt_name(), &serde_json::to_string(&self.dirs).unwrap()) as usize;

        if self.tokens > config.get_max_tokens() {
            println!("Too many tokens: {}", self.tokens);
            std::process::exit(1);
        } else {
            println!("Tokens: {}", self.tokens);
        }
    }

    pub fn check(&mut self, config: &Config) {
        let path = get_exec_path().join("indexes.txt");
        let content = std::fs::read_to_string(path).expect("Indexes file error");
        let hash: String = hashing(&content);
        if (self.time < Utc::now().timestamp() - 60 * 60 * 24) || self.hash != hash {
            self.scan(config);
            self.save();
        }
    }
}

impl Default for Indexes {
    fn default() -> Self {
        Self {
            time: 0,
            tokens: 0,
            hash: "".to_owned(),
            dirs: vec![],
        }
    }
}