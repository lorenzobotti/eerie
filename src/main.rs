use eerie::Files;
use std::path::PathBuf;
use std::env;
use std::fs;
use std::process;

const COMMANDS_GUIDE: &str = include_str!("usage.txt");

enum Command { Run, Create, Debug }

fn main() {
    let mut args = env::args();

    let command = match args.nth(1).or_crash(COMMANDS_GUIDE).to_lowercase().as_str() {
        "run" => Command::Run,
        "create" => Command::Create,
        "debug" => Command::Debug,
        _ => crash("unknown command"),
    };
    let docfile = args.next().or_crash(COMMANDS_GUIDE);
    let destination = args.next()
        .map(PathBuf::from)
        .unwrap_or(env::current_dir().or_crash("can't get current directory"));

    let docfile = fs::read_to_string(docfile).or_crash("can't read docfile");
    let (files, _) = match Files::from_str(&docfile) {
        Ok(files) => files,
        Err(error) => crash(error),
    };

    let (result, verb) = match command {
        Command::Run => (files.run(&destination), "running"),
        Command::Create => (files.create(&destination), "creating"),
        Command::Debug => ({dbg!(files); Ok(())}, "debugging"),
    };

    match result {
        Ok(_) => {},
        Err(error) => {
            eprintln!("error while {} folder: {}", verb, error);
            process::exit(1)
        },
    }
}


trait OrCrash<T> {
    fn or_crash(self, message: &str) -> T;
}

impl<T> OrCrash<T> for Option<T> {
    fn or_crash(self, message: &str) -> T {
        match self {
            Some(item) => item,
            None => crash(message),
        }
    }
}

impl<T, E> OrCrash<T> for Result<T, E> {
    fn or_crash(self, message: &str) -> T {
        match self {
            Ok(item) => item,
            Err(_) => crash(message),
        }
    }
}

fn crash(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}