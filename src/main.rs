use crossterm::{
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::{
    env, fs,
    io::{self, stdin, stdout, Write},
    path::Path,
    process::Command,
};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HOME: String = home::home_dir().unwrap().to_string_lossy().to_string();
}

fn main() -> io::Result<()> {
    loop {
        let dir = {
            //TODO: WTF is this?
            let current_dir = env::current_dir()?
                .as_os_str()
                .to_string_lossy()
                .to_string();
            current_dir.replace(HOME.as_str(), "~")
        };
        queue!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print(dir),
            SetForegroundColor(Color::Red),
            Print(" ❯ "),
            ResetColor,
        )?;
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // everything after the first whitespace character
        //     is interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                if let Some(new_dir) = args.peekable().peek() {
                    let new_dir = new_dir.replace('~', HOME.as_str());
                    let root = Path::new(&new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
            }
            "ls" => {
                let paths = fs::read_dir(env::current_dir()?)?;

                for path in paths {
                    print!("{} ", path.unwrap().file_name().to_string_lossy())
                }
                println!();
            }
            "clear" | "cls" => {
                Command::new("cmd")
                    .args(["/C", "cls"])
                    .status()
                    .expect("failed to execute process");
            }
            "exit" | "quit" => return Ok(()),
            command => {
                let child = Command::new(command).args(args).spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => {
                        child.wait()?;
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
