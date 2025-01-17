use std::{env, fs, io};
use std::io::{Write, BufRead};
use std::fs::OpenOptions;
use chrono::Local;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: log <command> [name]");
        eprintln!("Commands: new, view, edit, delete, restore");
        return;
    }

    let command = args[1].as_str();
    let log_file = "logs.txt";
    let backup_file = "logs_backup.txt";

    match command {
        "new" => {
            if args.len() < 3 {
                eprintln!("Please provide a name for the log.");
                return;
            }
            let name = &args[2];
            new_log(log_file, name);
        }
        "view" => view_logs(log_file),
        "edit" => edit_log(log_file),
        "delete" => delete_log(log_file, backup_file),
        "restore" => restore_logs(log_file, backup_file),
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn new_log(file: &str, name: &str) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut log_entry = format!("\nLog: {}\nDate: {}\n", name, date);

    println!("Enter Goals for the Week (type 'END' to finish):");
    log_entry.push_str("Goals for the Week:\n");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("END") {
            break;
        }
        log_entry.push_str(&format!("- {}\n", line));
    }

    println!("Enter Progress (type 'END' to finish):");
    log_entry.push_str("Progress:\n");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("END") {
            break;
        }
        log_entry.push_str(&format!("- {}\n", line));
    }

    println!("Enter Challenges (type 'END' to finish):");
    log_entry.push_str("Challenges:\n");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("END") {
            break;
        }
        log_entry.push_str(&format!("- {}\n", line));
    }

    println!("Enter Feedback from Mentor (type 'END' to finish):");
    log_entry.push_str("Feedback from Mentor:\n");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("END") {
            break;
        }
        log_entry.push_str(&format!("- {}\n", line));
    }

    println!("Enter Next Steps (type 'END' to finish):");
    log_entry.push_str("Next Steps:\n");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("END") {
            break;
        }
        log_entry.push_str(&format!("- {}\n", line));
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .expect("Unable to open or create the file.");

    file.write_all(log_entry.as_bytes()).expect("Unable to write to the file.");
    println!("Log saved successfully!");
}

fn view_logs(file: &str) {
    match fs::read_to_string(file) {
        Ok(content) => println!("\nLogs:\n{}", content),
        Err(_) => println!("No logs found."),
    }
}

fn edit_log(file: &str) {
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => {
            println!("No logs found to edit.");
            return;
        }
    };

    let logs: Vec<&str> = content.split("\n\n").collect();
    for (i, log) in logs.iter().enumerate() {
        println!("Log {}:\n{}", i + 1, log);
    }

    println!("Enter the log number to edit:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let log_index: usize = input.trim().parse().unwrap_or(0);

    if log_index == 0 || log_index > logs.len() {
        println!("Invalid log number.");
        return;
    }

    let mut selected_log = logs[log_index - 1].to_string();

    println!("Which section would you like to edit? (goals, progress, challenges, feedback, next steps):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let section = input.trim().to_lowercase();

    println!("Enter the updated content (type 'END' to finish):");
    let mut updated_section = String::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("END") {
            break;
        }
        updated_section.push_str(&format!("- {}\n", line));
    }

    selected_log = update_section(&selected_log, &section, &updated_section);

    let mut updated_logs = logs.clone();
    updated_logs[log_index - 1] = &selected_log;
    fs::write(file, updated_logs.join("\n\n")).expect("Unable to write to the file.");
    println!("Log updated successfully!");
}

fn update_section(log: &str, section: &str, new_content: &str) -> String {
    let section_header = match section {
        "goals" => "Goals for the Week:",
        "progress" => "Progress:",
        "challenges" => "Challenges:",
        "feedback" => "Feedback from Mentor:",
        "next steps" => "Next Steps:",
        _ => {
            println!("Invalid section.");
            return log.to_string();
        }
    };

    let mut updated_log = String::new();
    let mut inside_section = false;
    let mut found_section = false;

    for line in log.lines() {
        if line.trim() == section_header {
            updated_log.push_str(&format!("{}\n{}", section_header, new_content));
            inside_section = true;
            found_section = true;
        } else if inside_section && (line.trim().is_empty() || line.starts_with(char::is_uppercase)) {
            inside_section = false;
            if !line.trim().is_empty() {
                updated_log.push_str(&format!("{}\n", line));
            }
        } else if !inside_section {
            updated_log.push_str(&format!("{}\n", line));
        }
    }

    if !found_section {
        updated_log.push_str(&format!("{}\n{}", section_header, new_content));
    }

    updated_log
}

fn delete_log(file: &str, backup_file: &str) {
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => {
            println!("No logs found to delete.");
            return;
        }
    };

    let logs: Vec<&str> = content.split("\n\n").collect();
    for (i, log) in logs.iter().enumerate() {
        println!("Log {}:\n{}", i + 1, log);
    }

    println!("Enter the log number to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let log_index: usize = input.trim().parse().unwrap_or(0);

    if log_index == 0 || log_index > logs.len() {
        println!("Invalid log number.");
        return;
    }

    println!("Are you sure you want to delete Log {}? (yes/no):", log_index);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "yes" {
        println!("Deletion canceled.");
        return;
    }

    // Backup existing logs
    fs::write(backup_file, &content).expect("Unable to back up logs.");

    let mut updated_logs = logs.clone();
    updated_logs.remove(log_index - 1);

    fs::write(file, updated_logs.join("\n\n")).expect("Unable to update the log file.");
    println!("Log {} deleted successfully.", log_index);
}

fn restore_logs(file: &str, backup_file: &str) {
    match fs::read_to_string(backup_file) {
        Ok(backup_content) => {
            fs::write(file, backup_content).expect("Unable to restore logs from backup.");
            println!("Logs restored successfully.");
        }
        Err(_) => println!("No backup found to restore."),
    }
}