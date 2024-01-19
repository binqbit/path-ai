use serde::{Serialize, Deserialize};

use crate::{get_exec_path, DEFAULT_GPT_MODEL};



#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub apikey: Option<String>,
    gpt_model: Option<String>,
}


impl Config {
    pub fn load() -> Option<Self> {
        let path = get_exec_path().join("config.json");
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(config) => {
                        Some(config)
                    },
                    Err(_) => {
                        println!("Failed to parse 'config.json'");
                        None
                    },
                }
            },
            Err(_) => {
                println!("Failed to read 'config.json'");
                None
            },
        }
    }

    pub fn save(&self) {
        let path = get_exec_path().join("config.json");
        if std::fs::write(path, serde_json::to_string_pretty(self).unwrap()).is_err() {
            println!("Failed to save 'config.json'");
        }
    }

    pub fn set_apikey(&mut self, apikey: String) {
        self.apikey = Some(apikey);
        self.save();
    }

    pub fn set_gpt_model(&mut self, gpt_model: String) -> bool {
        if gpt_model == "gpt-3.5-turbo-1106" || gpt_model == "gpt-4-1106-preview" {
            self.gpt_model = Some(gpt_model);
            self.save();
            true
        } else {
            false
        }
    }

    pub fn get_gpt_model(&self) -> &str {
        self.gpt_model.as_ref().map(|m| m.as_str()).unwrap_or(DEFAULT_GPT_MODEL)
    }

    pub fn get_gpt_name(&self) -> &str {
        match self.get_gpt_model() {
            "gpt-3.5-turbo-1106" => "gpt-3.5",
            "gpt-4-1106-preview" => "gpt-4",
            _ => "gpt-3.5",
        }
    }

    pub fn get_max_tokens(&self) -> usize {
        match self.get_gpt_model() {
            "gpt-3.5-turbo-1106" => 15000,
            "gpt-4-1106-preview" => 50000,
            _ => 3000,
        }
    }
}