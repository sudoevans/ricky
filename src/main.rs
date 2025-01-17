use std::{env, fs, io};
use std::io::{Write, BufRead};
use std::fs::OpenOptions;
use std::path::PathBuf;
use chrono::Local;

fn get_data_directory() -> PathBuf {
    let mut dir = PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()));
    dir.push(".log_manager");
    fs::create_dir_all(&dir).expect("Failed to create data directory");
    dir
}

fn get_file_path(filename: &str) -> PathBuf {
    let mut path = get_data_directory();
    path.push(filename);
    path
}


// Parse logs


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: log <command> [name]");
        eprintln!("Commands: new, view, edit, delete, restore");
        return;
    }

    let command = args[1].as_str();
    let log_file = get_file_path("logs.txt");
    let backup_file = get_file_path("logs_backup.txt");

    match command {
        "new" => {
            if args.len() < 3 {
                eprintln!("Please provide a name for the log.");
                return;
            }
            let name = &args[2];
            new_log(&log_file, name);
        }
        "view" => view_logs(&log_file),
        "edit" => edit_log(&log_file),
        "delete" => delete_log(&log_file, &backup_file),
        "restore" => restore_logs(&log_file, &backup_file),
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn new_log(file: &PathBuf, name: &str) {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut log_entry = format!("\nLog: {}\nDate: {}\n", name, date);

    let sections = [
        "Goals for the Week",
        "Progress",
        "Challenges",
        "Feedback from Mentor",
        "Next Steps"
    ];

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    for section in sections.iter() {
        println!("Enter {} (type 'END' to finish):", section);
        log_entry.push_str(&format!("{}:\n", section));
        
        loop {
            let mut line = String::new();
            handle.read_line(&mut line).expect("Failed to read input");
            let trimmed = line.trim();
            
            if trimmed.eq_ignore_ascii_case("END") {
                break;
            }
            
            log_entry.push_str(&format!("- {}\n", trimmed));
        }
        log_entry.push('\n');
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .expect("Unable to open or create the file");

    file.write_all(log_entry.as_bytes()).expect("Unable to write to the file");
    println!("Log saved successfully!");
}

fn parse_logs(content: &str) -> Vec<String> {
    let mut logs = Vec::new();
    let mut current_log = String::new();
    let mut is_first = true;

    for line in content.lines() {
        if line.starts_with("Log: ") {
            if !is_first {
                if !current_log.trim().is_empty() {
                    logs.push(current_log);
                }
                current_log = String::new();
            }
            is_first = false;
        }
        current_log.push_str(line);
        current_log.push('\n');
    }

    if !current_log.trim().is_empty() {
        logs.push(current_log);
    }

    logs
}

fn view_logs(file: &PathBuf) {
    match fs::read_to_string(file) {
        Ok(content) => {
            if content.trim().is_empty() {
                println!("No logs found.");
                return;
            }

            let logs = parse_logs(&content);
            println!("\nFound {} weekly logs:\n", logs.len());
            
            for (i, log) in logs.iter().enumerate() {
                println!("Weekly Log #{}:\n{}\n{}", i + 1, log, "-".repeat(50));
            }
        }
        Err(_) => println!("No logs found."),
    }
}

fn edit_log(file: &PathBuf) {
    let content = match fs::read_to_string(file) {
        Ok(content) => {
            if content.trim().is_empty() {
                println!("No logs found to edit.");
                return;
            }
            content
        }
        Err(_) => {
            println!("No logs found to edit.");
            return;
        }
    };

    let logs = parse_logs(&content);
    
    println!("\nAvailable weekly logs:");
    for (i, log) in logs.iter().enumerate() {
        println!("\nWeekly Log #{}:\n{}", i + 1, log);
        println!("{}", "-".repeat(50));
    }

    println!("\nEnter the weekly log number to edit (1-{}):", logs.len());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let log_index: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= logs.len() => num,
        _ => {
            println!("Invalid log number.");
            return;
        }
    };

    println!("\nWhich section would you like to edit?");
    println!("1. Goals for the Week");
    println!("2. Progress");
    println!("3. Challenges");
    println!("4. Feedback from Mentor");
    println!("5. Next Steps");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let section = match input.trim() {
        "1" => "goals",
        "2" => "progress",
        "3" => "challenges",
        "4" => "feedback",
        "5" => "next steps",
        _ => {
            println!("Invalid section.");
            return;
        }
    };

    println!("\nEnter the updated content (type 'END' to finish):");
    let mut updated_section = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    loop {
        let mut line = String::new();
        handle.read_line(&mut line).expect("Failed to read input");
        let trimmed = line.trim();
        
        if trimmed.eq_ignore_ascii_case("END") {
            break;
        }
        
        updated_section.push_str(&format!("- {}\n", trimmed));
    }

    let mut updated_logs = logs;
    let new_log = update_section(&updated_logs[log_index - 1], section, &updated_section);
    updated_logs[log_index - 1] = new_log;
    
    fs::write(file, updated_logs.join("\n")).expect("Unable to write to the file");
    println!("Log updated successfully!");
}

fn update_section(log: &str, section: &str, new_content: &str) -> String {
    let section_header = match section {
        "goals" => "Goals for the Week:",
        "progress" => "Progress:",
        "challenges" => "Challenges:",
        "feedback" => "Feedback from Mentor:",
        "next steps" => "Next Steps:",
        _ => return log.to_string(),
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
            updated_log.push_str(line);
            updated_log.push('\n');
        } else if !inside_section {
            updated_log.push_str(line);
            updated_log.push('\n');
        }
    }

    if !found_section {
        updated_log.push_str(&format!("\n{}\n{}", section_header, new_content));
    }

    updated_log
}

fn delete_log(file: &PathBuf, backup_file: &PathBuf) {
    let content = match fs::read_to_string(file) {
        Ok(content) => {
            if content.trim().is_empty() {
                println!("No logs found to delete.");
                return;
            }
            content
        }
        Err(_) => {
            println!("No logs found to delete.");
            return;
        }
    };

    let logs = parse_logs(&content);
    
    println!("\nAvailable weekly logs:");
    for (i, log) in logs.iter().enumerate() {
        println!("\nWeekly Log #{}:\n{}", i + 1, log);
        println!("{}", "-".repeat(50));
    }

    println!("\nEnter the weekly log number to delete (1-{}):", logs.len());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let log_index: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= logs.len() => num,
        _ => {
            println!("Invalid log number.");
            return;
        }
    };

    println!("Are you sure you want to delete Weekly Log {}? (yes/no):", log_index);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    if input.trim().to_lowercase() != "yes" {
        println!("Deletion canceled.");
        return;
    }

    // Backup existing logs
    fs::write(backup_file, &content).expect("Unable to back up logs");

    let mut updated_logs = logs;
    updated_logs.remove(log_index - 1);

    fs::write(file, updated_logs.join("\n")).expect("Unable to update the log file");
    println!("Weekly Log {} deleted successfully.", log_index);
}
fn restore_logs(file: &PathBuf, backup_file: &PathBuf) {
    match fs::read_to_string(backup_file) {
        Ok(backup_content) => {
            fs::write(file, backup_content).expect("Unable to restore logs from backup");
            println!("Logs restored successfully.");
        }
        Err(_) => println!("No backup found to restore."),
    }
}