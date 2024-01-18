use std::{env, path::{PathBuf, Path}};



pub const PATH_AI_VERSION: &str = "v1.0.0";
pub const PATH_AI_GPT_MODEL: &str = "gpt-3.5-turbo-1106";
pub const TIKTOKEN_MODEL: &str = "gpt-3.5";
pub const MAX_TOKENS: usize = 15000;

lazy_static! {
    pub static ref EXE_PATH: PathBuf = env::current_exe().unwrap();
}

pub fn get_exec_path<'a>() -> &'a Path {
    EXE_PATH.parent().unwrap()
}