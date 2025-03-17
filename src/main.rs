use std::{env, fs, io::{self, BufRead, Write}, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_prompt(){
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        let bytes = stdin.lock().read_line(&mut line).unwrap();

        if bytes == 0 {
            break;
        }

        run(line);
    }
}

fn run_file(path: &str) {
    let content = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Could not read file '{}': {}", path, err);
        std::process::exit(65);
    });

    run(content);
}

fn run(source: String) {
    // TODO: placeholder
    println!("running {}", source);
}
