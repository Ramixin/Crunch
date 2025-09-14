use std::io::Write as IoWrite;
use std::fmt::Write as FmtWrite;
use std::{env, fs};
use std::fs::File;
use std::process::exit;
use colored::Colorize;
use std::path::{Path as IoPath, PathBuf};
use std::io::{Read, Result as IoResult};
use crate::lexer::{ToTokens, TokenEntry};
use crate::types::Function;

mod lexer;
mod types;
mod parser;
mod statements;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    /* simulate arg push */
    args.remove(0);
    args.push(String::from("scripts/test.py"));
    /* ----------------- */

    if args.len() != 1 {
        error("Usage: <source dir/file path>");
    }

    match compile(args) {
        Ok(_) => {}
        Err(e) => error(&e),
    }
}

fn compile(mut args: Vec<String>) -> Result<(), String> {
    let input_path_string = args.remove(0);
    let input_path = IoPath::new(&input_path_string);
    if !input_path.exists() {
        return Err(format!("source path '{}' does not exist", input_path_string));
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    let _ = walk_dir(input_path, &mut paths);
    let mut names : Vec<&str> = Vec::new();
    let mut contents : Vec<String> = Vec::new();
    for path in &paths {
        let name = path.file_name().unwrap().to_str().unwrap();
        println!("appending name {}", name);
        names.push(name);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(format!("failed to open file: {}", e))
        };
        let mut content = String::new();
        unwrap_io(std::io::stdout().flush());
        match file.read_to_string(&mut content) {
            Ok(_) => {}
            Err(_) => return Err(format!("failed to read file '{}'", name))
        }
        contents.push(content);
    }
    println!("{:?}", names);

    let mut tokens : Vec<Vec<TokenEntry>> = Vec::new();
    for (i, content) in contents.into_iter().enumerate() {
        unwrap_io(std::io::stdout().flush());
        let token_vec: Vec<TokenEntry> = match content.to_tokens() {
            Ok(v) => v,
            Err(e) => return Err(format!("failed to tokenize file \"{}\": {}", names[i], e))
        };
        tokens.push(token_vec);
    }

    let mut programs : Vec<Vec<Function>> = Vec::new();
    for vec in tokens {
        programs.push(parser::parse_tokens(vec)?);
    }
    for program in programs {
        println!("{:#?}", program);
    }
    Ok(())
}

fn error(message: &str) -> ! {
    eprintln!("\n{}: {}", "Crunch Error".red(), message);
    exit(1);
}

fn walk_dir(path: &IoPath, files : &mut Vec<PathBuf>) -> IoResult<()> {
    if path.is_file() {
        match path.extension() {
            Some(ext) => {
                let ext = ext.to_str().unwrap();
                if ext != "py" && ext != "python" {
                    error(&format!("expected file extension .py or .python, but found '{:?}'", ext));
                }
            }
            None => {
                error(&format!("file '{}' does not have an extension", path.display()));
            }
        }
        files.push(path.to_path_buf());
        return Ok(())
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let _ = walk_dir(&path, files)?;
    }
    Ok(())
}

fn unwrap_io<T>(res : IoResult<T>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => error(format!("io error: {}", e).as_str()),
    }
}
