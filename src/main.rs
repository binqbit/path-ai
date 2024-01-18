
#[macro_use]
extern crate lazy_static;

pub mod path_ai;
pub mod utils;

pub use path_ai::*;
pub use utils::*;



fn main() {
    let mut args = std::env::args();
    args.next();

    let mut description = if let Some(arg) = args.next() {
        if arg == "--key" || arg == "-k" {
            let key = args.next().expect("Missing openai key");
            Config::set_apikey(key);
            return;
        } else if arg == "--version" || arg == "-v" {
            println!("version: {PATH_AI_VERSION}");
            println!("gpt model: {PATH_AI_GPT_MODEL}");
            return;
        } else if arg == "--help" || arg == "-h" {
            println!(r#"
[--key, -k] - set openai key
path-ai --key 1234567890qwertyuiopasdfghjklzxcvbnm

[--version, -v] - view path-ai version
path-ai --version

[--help, -h] - view help
path-ai --help

path-ai [description of the folder and program in which you want to open it]
path-ai terminal on desktop

Create indexes.txt file in the same folder as path-ai.exe
indexes.txt format:
```
C:/::1*
D:/::2*
C:/Users/user/Desktop/::1*
```
"#);
            return;
        } else {
            arg
        }
    } else {
        return;
    };

    while let Some(arg) = args.next() {
        description = format!("{description} {arg}");
    }

    if let Some(config) = Config::load() {
        let gpt = ChatGPT::new(&config);
        path_ai(&gpt, description.trim().to_string());
    } else {
        println!("Missing openai key!");
    }
}
