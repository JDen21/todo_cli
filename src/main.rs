mod command_parser;

use clap::ArgMatches;
use home::home_dir;
use std::fs;
use  command_parser::get_match_result;
use chrono::{Local, Datelike};


fn main(){
    let match_result = get_match_result();
    match match_result.subcommand_name() {
        Some("add") => {
            // * it should be okay to use unwrap here bec, 'add' is sure in use
            add_todo(&match_result.subcommand_matches("add").unwrap());
        },
        Some("done") => {
            // * it should be okay to use unwrap here bec, 'done' is sure in use
            done_todo(&match_result.subcommand_matches("done").unwrap());
        },
        Some("clear") => {
            // * it should be okay to use unwrap here bec, 'done' is sure in use
            clear_todo(&match_result.subcommand_matches("clear").unwrap());
        },
        None => {
            show_todo(&match_result);
        },
        _ => {panic!()}
    };
}

fn show_todo(match_result: &ArgMatches){
    let mut line_arr: Vec<String> = vec![];
    let data = fs::read_to_string(file_path());
    if let Ok(lines) = data {
        let mut to_append: Vec<String> = lines
        .split(", \n")
        .map(|line| line.to_string())
        .collect();
        line_arr.append(&mut to_append);
    } else {
        println!("read file failed.");
        createdir_fallback();
        show_todo(match_result);
        return ();
    }

    if !match_result.get_flag("on-going") 
    && !match_result.get_flag("finished") {
        for (index, line) in line_arr.into_iter().enumerate() {
            if line.len() == 0 {
                continue;
            }
            println!("{} -> {}", index, line.replace(",", " - "));
        }
        return ();
    }

    if match_result.get_flag("on-going"){
        for line in line_arr.into_iter() {
            if !line.ends_with("done\n") {
                println!("{}", line.replace(",", " - "));   
            }
        }
        return ();
    }

    if match_result.get_flag("finished") {
        for line in line_arr.into_iter() {
            if line.ends_with("done\n") {
                println!("{}", line.replace(",", " - "));   
            }
        }
    }
    return ();
}

fn add_todo(match_result: &ArgMatches) {
    let mut new_todo: String = String::from("");
    let mut file_content: String = String::from("");

    if let Some(value) = match_result.get_one::<String>("name") {
        println!("inserting todo {}", value);
        new_todo.push_str(value);
    } else {
        println!("unable to read new todo.");
        return ();
    }

    if new_todo.contains(",") {
        println!("unable to add todo due to  ','");
        return ();
    }
    let current_time = Local::now();
    let new_todo = new_todo + "," + format!("{}", current_time.format("%m/%d/%Y,")).as_str();
    let new_todo = new_todo + format!("{}", current_time.date_naive().weekday()).as_str();
    if let Ok(value) = fs::read_to_string(file_path()) {
        file_content.push_str(value.as_str());
    } else {
        println!("add read file failed.");
        createdir_fallback();
        add_todo(match_result);
        return ();
    }
    file_content.push_str(new_todo.as_str());
    file_content.push_str(", \n");
    
    if let Ok(_) = fs::write(file_path(), file_content) {
        return ();
    } else {
        println!("add write file failed.");
    }
}

fn done_todo(match_result: &ArgMatches) {
    let mut todo_to_update: String = String::from("");
    let mut file_content: String = String::from("");

    if let Ok(value) = fs::read_to_string(file_path()) {
        file_content.push_str(value.as_str());
    } else {
        println!("unable to read file.");
        createdir_fallback();
        done_todo(match_result);
        return ();
    }

    if let Some(value) = match_result.get_one::<String>("name") {
        todo_to_update.push_str(value);
    } else if let Some(value) = match_result.get_one::<String>("index") {
        if let Ok(parsed_value) = value.parse::<usize>() {
            let todo_name = file_content.clone()
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| line.split(",").collect::<Vec<&str>>())
            .enumerate()
            .filter(|(idx, _cells)| *idx == parsed_value )
            .fold(String::from(""), |_acc, (_idx, cells): (usize, Vec<&str>)| cells[0].to_string());
        todo_to_update = todo_name;
        }
    } else {
        println!("unable to update.");
    }

    let file_lines: String = file_content
    .split("\n")
    .collect::<Vec<&str>>()
    .iter()
    .map(|line| line.split(",").collect::<Vec<&str>>())
    .map(|mut cells: Vec<&str>| {
        if cells[0] == todo_to_update {
            cells.push("done");
            // * because Im too lazy to debug
            return cells.join(",").replace(", ,", ",");
        }
        return cells.join(",");
    })
    .collect::<Vec<String>>()
    .join("\n");

    if let Ok(_) = fs::write(file_path(), &file_lines) {
        println!("{}", file_lines.replace(",", " - "));
    } else {
        println!("unable to write file.");
    }
}

fn clear_todo(match_result: &ArgMatches) {
    let mut clear_single = false;
    let mut to_delete:String = String::from("");
    let mut file_content: String = String::from("");

    if let Ok(value) = fs::read_to_string(file_path()) {
        file_content.push_str(value.as_str());
    } else {
        println!("unable to read file.");
        createdir_fallback();
        done_todo(match_result);
        return ();
    }

    if let Some(name) = match_result.get_one::<String>("name") {
        clear_single = true;
        to_delete.push_str(name);
    } 
    // * basically getting the index of the todo 
    else if let Some(value) = match_result.get_one::<String>("index") {
        if let Ok(parsed_value) = value.parse::<usize>() {
            let todo_name = file_content.clone()
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| line.split(",").collect::<Vec<&str>>())
            .enumerate()
            .filter(|(idx, _cells)| *idx == parsed_value )
            .fold(String::from(""), |_acc, (_idx, cells): (usize, Vec<&str>)| cells[0].to_string());
         to_delete = todo_name;
         clear_single = true;
        }
    }

    if clear_single {
        let mut all_todo: String = String::from("");
        if let Ok(value) = fs::read_to_string(file_path()) {
            all_todo.push_str(value.as_str());
        } else {
            println!("unable to read file.");
            return ();
        }
        let all_todo = all_todo.split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|line| !line.starts_with(to_delete.as_str()))
        .map(|line| *line)
        .collect::<Vec<&str>>()
        .join("\n");
        if let Ok(_) = fs::write(file_path(), &all_todo) {
            println!("removed: {}", &to_delete);
        } else {
            println!("unable to write file.");
        }
        return ();
    }

    let response = fs::write(file_path(), "");
    if let Ok(_) = response {
        println!("clearing todo.");
    } else {
        println!("unable to clear todo.");
        createdir_fallback();
        clear_todo(match_result);
        return ();
    }
}

fn file_path() -> String {
    if let Some(path) = home_dir() {
        format!("{}/u/todo.txt", path.display())
    } else {
        // * fallback to saving in terminal path
        "./".to_string()
    }
}

fn createdir_fallback() {
    let mut home: String = String::new();
    if let Some(path) = home_dir() {
        let root_path: String = format!("{}", path.display());
        home.push_str(root_path.as_str());
    }
    let folder_loc: String = format!("{}/{}", home, "u".to_string());
    if let Ok(_) = fs::create_dir_all(&folder_loc) {
        println!("directory sample created.");
    } else {
        println!("unable to create folder.")
    }
    let file_loc = format!("{}/{}", &folder_loc, "todo.txt".to_string());
    if let Ok(_) = fs::write(file_loc, "") {
        println!("file created todo.txt");
    } else {
        println!("unable to create file todo.txt");
    }
}
