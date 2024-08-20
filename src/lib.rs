use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::PathBuf;

use colored::Colorize;

pub fn list() {
    let mut count: i8 = 0;
    read_to_string(get_zsh_path()).expect(&*"Could not read .zshrc file".red().bold()).lines()
        .filter(|line| line.starts_with("alias"))
        .for_each(|line| {
            let alias = line.split_once(" ")
                .expect(&*"Failed to read aliases".red().bold()).1.split("=")
                .collect::<Vec<&str>>();
            print_alias(alias[0].to_string(), alias[1].to_string());
            count += 1;
        });
    print_alias_search_result(count)
}

pub fn search(query: Option<String>) {
    if query.is_none() {
        println!("{}", "No query provided".red().bold());
        return;
    }
    let mut count = 0;
    read_to_string(get_zsh_path()).expect(&*"Could not read .zshrc file".red().bold()).lines()
        .filter(|line| line.starts_with("alias"))
        .for_each(|line| {
            let alias = line.split_once(" ")
                .expect(&*"Failed to read aliases".red().bold()).1.split("=")
                .collect::<Vec<&str>>();
            if alias[0].contains(query.clone().unwrap().as_str()) {
                print_alias(alias[0].to_string(), alias[1].to_string());
                count += 1
            }
        });
    print_alias_search_result(count)
}

pub fn add(alias: Option<String>, command: Option<String>) {
    if alias.is_none() {
        println!("{}", "No alias provided".red().bold());
        return;
    }
    if command.is_none() {
        println!("{}", "No command provided".red().bold());
        return;
    }
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(get_zsh_path());

    if file.is_err() {
        println!("{}", "Could not open .zshrc file".red().bold());
        return;
    }

    for line in read_to_string(get_zsh_path()).unwrap().lines() {
        if line.starts_with("alias") {
            let aliases = line.split_once(" ").unwrap().1.split("=").collect::<Vec<&str>>();
            if aliases[0].eq(&alias.clone().unwrap()) {
                println!("Alias '{}' already exists", alias.clone().unwrap().red().bold());
                return;
            }
        }
    }
    let result = writeln!(file.unwrap(), "{}", format!("alias {}='{}'", alias.unwrap(), command.unwrap()));

    if result.is_err() {
        println!("{}", "Failed to add alias".red().bold());
        return;
    }
    println!("{}", "Alias added successfully".green().bold());
}

pub fn update(alias: Option<String>, command: Option<String>) {
    if alias.is_none() {
        println!("{}", "No alias provided".red().bold());
        return;
    }
    if command.is_none() {
        println!("{}", "No command provided".red().bold());
        return;
    }

    let mut lines: Vec<String> = Vec::new();

    for line in read_to_string(get_zsh_path()).unwrap().lines() {
        if line.starts_with("alias") {
            let aliases = line.split_once(" ").unwrap().1.split("=").collect::<Vec<&str>>();
            if aliases[0].eq(&alias.clone().unwrap()) {
                lines.push(format!("alias {}='{}'", aliases[0], command.clone().unwrap()));
            } else {
                lines.push(String::from(line));
            }
        } else {
            lines.push(String::from(line));
        }
    }
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_zsh_path()) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", "Could not open .zshrc file".red().bold());
            return;
        }
    };

    for line in lines {
        if let Err(_) = writeln!(file, "{}", line) {
            println!("{}", "Failed to write to .zshrc file".red().bold());
            return;
        }
    }
    println!("{}", "Successfully updated alias".green().bold());
}

pub fn rename(alias: Option<String>, new_name: Option<String>) {
    if alias.is_none() {
        println!("{}", "No alias provided".red().bold());
        return;
    }
    if new_name.is_none() {
        println!("{}", "No new name provided".red().bold());
        return;
    }

    let mut lines: Vec<String> = Vec::new();

    for line in read_to_string(get_zsh_path()).unwrap().lines() {
        if line.starts_with("alias") {
            let aliases = line.split_once(" ").unwrap().1.split("=").collect::<Vec<&str>>();
            if aliases[0].eq(&alias.clone().unwrap()) {
                lines.push(format!("alias {}='{}'", new_name.clone().unwrap(), aliases[1].replace("'", "")));
            } else {
                lines.push(String::from(line));
            }
        } else {
            lines.push(String::from(line));
        }
    }
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_zsh_path()) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", "Could not open .zshrc file".red().bold());
            return;
        }
    };

    for line in lines {
        if let Err(_) = writeln!(file, "{}", line) {
            println!("{}", "Failed to write to .zshrc file".red().bold());
            return;
        }
    }
    println!("{}", "Successfully renamed alias".green().bold());
}

pub fn remove(alias: Option<String>) {
    if alias.is_none() {
        println!("{}", "No alias provided".red().bold());
        return;
    }

    let mut lines: Vec<String> = Vec::new();

    for line in read_to_string(get_zsh_path()).unwrap().lines() {
        if line.starts_with("alias") {
            let aliases = line.split_once(" ").unwrap().1.split("=").collect::<Vec<&str>>();
            if !aliases[0].eq(&alias.clone().unwrap()) {
                lines.push(String::from(line));
            }
        } else {
            lines.push(String::from(line));
        }
    }
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_zsh_path()) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", "Could not open .zshrc file".red().bold());
            return;
        }
    };

    for line in lines {
        if let Err(_) = writeln!(file, "{}", line) {
            println!("{}", "Failed to write to .zshrc file".red().bold());
            return;
        }
    }
    println!("{}", "Successfully removed alias".green().bold());
}

fn get_zsh_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let mut zshrc_path = PathBuf::from(home_dir);
    zshrc_path.push(".zshrc");

    return zshrc_path;
}

fn print_alias(command: String, instruction: String) {
    println!("{} {} {}", command.bright_green().bold(), String::from("=").bold().bright_white(), instruction.green().bold());
}

fn print_alias_search_result(count: i8) {
    println!("{} {}", count.to_string().blue().bold(), "aliases found".blue().bold());
}