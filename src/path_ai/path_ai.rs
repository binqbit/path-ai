use std::process::Command;
use serde::{Serialize, Deserialize};
use crate::{ChatGPT, Message, Indexes};



#[derive(Debug, Serialize, Deserialize, Clone)]
enum OpenAs {
    Terminal,
    Explorer,
    VsCode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenDir {
    path: String,
    open_as: OpenAs,
}


impl OpenDir {
    pub fn open(&self) {
        let mut child = match self.open_as {
            OpenAs::Terminal => {
                println!("Open in terminal: {}", self.path);
                if cfg!(target_os = "windows") {
                    Command::new("wt")
                        .arg("-w")
                        .args(&["cd", "/d", &self.path])
                        .spawn()
                        .expect("Failed to execute command")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .args(["cd", &self.path])
                        .spawn()
                        .expect("Failed to execute command")
                }
            },
            OpenAs::Explorer => {
                println!("Open in explorer: {}", self.path);
                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/C")
                        .args(["start", &self.path])
                        .spawn()
                        .expect("Failed to execute command")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .args(["xdg-open", &self.path])
                        .spawn()
                        .expect("Failed to execute command")
                }
            },
            OpenAs::VsCode => {
                println!("Open in vscode: {}", self.path);
                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/C")
                        .args(["code", &self.path])
                        .spawn()
                        .expect("Failed to execute command")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .args(["code", &self.path])
                        .spawn()
                        .expect("Failed to execute command")
                }
            },
        };
        
        let status = child.wait().expect("Failed to wait for command");
        if let Some(status) = status.code() {
            if status != 0 && status != 1 {
                println!("Failed to open dir: {}", self.path);
            }
        }
    }
}



pub fn path_ai(gpt: &ChatGPT, description: String) {
    let dirs = serde_json::to_string(&Indexes::load().dirs).unwrap();
    let messages = vec![
        Message::new("assistant", format!(r#"
dirs: {dirs}
find the path according to the description and return it and open as: {description}
result format:
```json
{{
    "path": "path/to/folder",
    "open_as": "Explorer|Terminal|VsCode"
}}
```
"#)),
    ];

    match gpt.send(messages) {
        Ok(output) => {
            if let Some(open_dir) = output.json::<OpenDir>() {
                open_dir.open();
            } else {
                println!("No response from ChatGPT");
            }
        },
        Err(err) => {
            eprintln!("Failed to get response from ChatGPT: {err}");
        },
    }
}