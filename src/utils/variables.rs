use std::{env, path::{PathBuf, Path}};



pub const PATH_AI_VERSION: &str = "v1.1.0";
pub const DEFAULT_GPT_MODEL: &str = "gpt-3.5-turbo-1106";

lazy_static! {
    pub static ref EXE_PATH: PathBuf = env::current_exe().unwrap();
}

pub fn get_exec_path<'a>() -> &'a Path {
    EXE_PATH.parent().unwrap()
}