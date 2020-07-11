extern crate glob;
use crate::regex::regex_parser;
use std::path::Path;
use glob::glob;
use std::env;
extern crate walkdir;
extern crate string_parser;
extern crate dirs;
use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
mod parser;
use crate::parser::*;
mod regex;
fn main() -> std::io::Result<()> {
    if env::args().last().unwrap() == "--regex" {

        let mut path = String::from(dirs::home_dir().unwrap().to_str().unwrap());
        path.push_str("/.cargo/todo_config");
        println!("{}",path);
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
            where P: AsRef<Path>, {
                let file = match File::open(filename){
                    Ok(line) => line,
                    Err(_) => {
                        println!("{}", "File '~/.cargo/todo_config' not found".red());
                        panic!();
                    }
                };
                Ok(io::BufReader::new(file).lines())
        }

        let mut regex = Vec::new();
        for line in read_lines(path)? {
            let line = line.unwrap();
            regex.push(line);
        }

        let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
        path.push_str("/**/*.rs");

        for entry in match glob(&path) {
            Ok(entry) => entry,
            Err(e) => {
                println!("Couldn't access files. Error {}", e);
                Err(e).unwrap()
            }
        } {
            let path = entry.unwrap();
            let path = path.to_str().unwrap();
            
            //execute each parsers on the current file
            // for p in &parsers {
            //         p.parse(path);
            // }
                regex_parser(path, regex.clone())?;
        }

    }
    else{
        //this vector containes all the parsers we want to execute
        let mut parsers : Vec<Parser> = vec!();
    
        let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
        path.push_str("/**/*.rs");
        
        //we add a parser looking for the //todo keyword
        parsers.push(Parser::new(String::from("//todo"), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &'\n' {return true;} else { return false}})));
        //we add a parser looking for the todo!() token
        parsers.push(Parser::new(String::from("todo!("), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &')' {return true;} else { return false}})));
        
        //loop on every file within the current dir
        for entry in match glob(&path) {
            Ok(entry) => entry,
            Err(e) => {
                println!("Couldn't access files. Error {}", e);
                Err(e).unwrap()
            }
        } {
            let path = entry.unwrap();
            let path = path.to_str().unwrap();
            
            //execute each parsers on the current file
            for p in &parsers {
                    p.parse(path);
            }
        }
    }
    
    Ok(())
    
}


// test zone
//todo refactor
// fn test(){
//     todo!("hey");
// }