use std::env;
use std::fs;
use lang_parser::*;

fn main() {


    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 3 {
        println!("You are missing an argument. Example {} <language file> <code file>.",
            args[0]);
        return;
    }

    let file_name = &args[1];

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let mut interp = Interpreter::new();
    println!("{:#?}", interp.add_interpreter(&contents));
    println!("{:#?}", interp);


    let file_name = &args[2];
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let parse = interp.parse(&"program".to_string(), &contents); 
    println!("{:#?}", parse);
}

#[cfg(test)]
mod tests;
