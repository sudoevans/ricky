# Ricky Log Manager

A command-line tool for managing weekly progress logs. Track your goals, progress, challenges, mentor feedback, and next steps in an organized way.

## Features
- Create new weekly logs with structured sections
- View all weekly logs
- Edit specific sections of existing logs
- Delete logs with automatic backup
- Restore logs from backup
- Secure storage in user's home directory

## Installation

1. Make sure you have Rust and Cargo installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone https://github.com/sudoevans/ricky.git
cd ricky
```

3. Run the installation script:
```bash
chmod +x install.sh
./install.sh
```

The program will be installed system-wide and can be accessed using the `ricky` command from any directory.

## Usage

### Creating a New Log

```bash
ricky new "Week 1"
```

This will prompt you to enter information for each section:
```
Enter Goals for the Week (type 'END' to finish):
- Complete the rpc scavenger hunt
- Connect with peers.
- Write Module  documentation 
- Prepare Questions
- Write an automation script.
END

Enter Progress (type 'END' to finish):
- 9/11 completed RPC Scavenger hunt
- Prepared Questions for Office hours calls.
END

Enter Challenges (type 'END' to finish):
- No major issues
- Test coverage below target
END

Enter Feedback from Mentor (type 'END' to finish):
- Good progress
- Prepare for next week
END

Enter Next Steps (type 'END' to finish):
- Prepare for week two
- Push Automation
- Review error handling
END
```

### Viewing Logs

```bash
ricky view
```

Example output:
```
Found 2 weekly logs:

Weekly Log #1:
Log: Week 1
Date: 2025-01-17 14:30:22
Goals for the Week:
- Complete the user authentication module
- Set up CI/CD pipeline
- Write API documentation

Progress:
- Implemented user login and registration
- Created basic CI workflow

Challenges:
- RPC hunt autogradder failling.

Feedback from Mentor:
- Good progress on auth module
- Consider adding more error handling

Next Steps:
- Fix database migration scripts
- Improve test coverage
- Read to chapter 8 bitcoin book
--------------------------------------------------

Weekly Log #2:
[...]
```

### Editing a Log

```bash
ricky edit
```

1. First, select the weekly log number you want to edit
2. Choose the section to edit:
   ```
   Which section would you like to edit?
   1. Goals for the Week
   2. Progress
   3. Challenges
   4. Feedback from Mentor
   5. Next Steps
   ```
3. Enter the new content for that section
4. End input with 'END'

### Deleting a Log

```bash
ricky delete
```

This will:
1. Show all available logs
2. Prompt for the log number to delete
3. Ask for confirmation
4. Create a backup before deletion
5. Remove the selected log

### Restoring Logs

If you accidentally delete a log, you can restore from the last backup:

```bash
ricky restore
```

## File Locations

- Logs are stored in: `~/.log_manager/logs.txt`
- Backups are stored in: `~/.log_manager/logs_backup.txt`

## Log Structure

Each log entry contains:
- Log name and date
- Goals for the Week
- Progress
- Challenges
- Feedback from Mentor
- Next Steps

## Command Summary

```bash
ricky new "Log Name"    # Create a new weekly log
ricky view             # View all logs
ricky edit             # Edit an existing log
ricky delete           # Delete a log (creates backup)
ricky restore          # Restore from last backup
```

## Tips

1. Always use meaningful names for your logs (e.g., "Week 1", "Sprint 2", etc.)
2. Review previous logs before creating new ones to maintain continuity
3. Back up the ~/.log_manager directory periodically
4. Use 'END' to finish input in any section
5. Each section can have multiple entries
6. Push your logs to github or upload as gists.

## Error Handling

The program handles various error cases:
- Invalid commands
- Missing log name
- Invalid log numbers
- Missing files
- Empty logs
- Invalid section selections


### Customizing the Command Name

You can customize the command name to whatever you prefer. For example, to change from `ricky` to `log-manager` or any other name:

If you want to change run command, here is how:

1. Edit the `Cargo.toml` file:
```toml
[[bin]]
name = "your-preferred-name"  # Change this to what you want
path = "src/main.rs"
```

2. Edit the `install.sh` script to use your new name:
```bash
# Copy the binary with new name
sudo cp target/release/your-preferred-name /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/your-preferred-name
```

3. Reinstall:
```bash
# Remove old binary if it exists
sudo rm /usr/local/bin/ricky

# Reinstall with new name
./install.sh
```

Now you can use your custom command name:
```bash
log-manager new "Week 1"
log-manager view
log-manager edit
```

Some suggested names you might consider:
- `log-manager`
- `weeklog`
- `progress-log`
- `devlog`
- `worklog`

## Contributing

Feel free to submit issues and enhancement requests!

## License

MIT License - See LICENSE file for details