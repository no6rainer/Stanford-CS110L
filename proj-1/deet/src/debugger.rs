use crate::debugger_command::DebuggerCommand;
use crate::inferior::{Inferior, Status};
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: Vec<usize>
}

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if let Some(mut inferior) = self.inferior.take() {
                        println!("Killing running inferior (pid {})", inferior.pid());
                        if let Err(e) = inferior.kill() {
                            println!("Error killing inferior: {}", e);
                        }
                    }

                    if let Some(mut inferior) = Inferior::new(&self.target, &args, &self.breakpoints) {
                        let status = match inferior.resume() {
                            Ok(status) => status,
                            Err(e) => {
                                println!("Error continuing inferior: {}", e);
                                continue;
                            }
                        };
                        self.inferior = Some(inferior);
                        self.handle_status(status);
                    } else {
                        println!("Error starting subprocess");
                    }
                },

                DebuggerCommand::Quit => {
                    if let Some(mut inferior) = self.inferior.take() {
                        println!("Killing running inferior (pid {})", inferior.pid());
                        if let Err(e) = inferior.kill() {
                            println!("Error killing inferior: {}", e);
                        }
                    }
                    return;
                },

                DebuggerCommand::Continue => {
                    if let Some(inferior) = self.inferior.as_mut() {
                        let status = match inferior.resume() {
                            Ok(status) => status,
                            Err(e) => {
                                println!("Error continuing inferior: {}", e);
                                self.inferior = None;
                                continue;
                            }
                        };
                        self.handle_status(status);
                    } else {
                        println!("No inferior is running");
                    }
                },

                DebuggerCommand::Backtrace => {
                    if let Some(inferior) = &self.inferior {
                        if let Err(e) = inferior.print_backtrace(&self.debug_data) {
                            println!("Error printing backtrace: {}", e);
                        }
                    } else {
                        println!("No inferior is running");
                    }
                },

                DebuggerCommand::Break(target) => {
                    let mut resolved_addr: Option<usize> = None;
                    
                    if target.starts_with('*') {
                        let addr_str = &target[1..];
                        match parse_address(addr_str) {
                            Some(addr) => resolved_addr = Some(addr),
                            None => println!("Invalid address: {}", addr_str)
                        };
                    } else if let Ok(line_number) = target.parse::<usize>() {
                        match self.debug_data.get_addr_for_line(None, line_number) {
                            Some(addr) => resolved_addr = Some(addr),
                            None => println!("Invalid line number!")
                        }
                    } else if let Some(addr) = self.debug_data.get_addr_for_function(None, &target) {
                        resolved_addr = Some(addr);
                    } else {
                        println!("Could not parse breakpoint target: {}", target);
                    }

                    if let Some(breakpoint) = resolved_addr {
                        let idx = self.breakpoints.len();
                        self.breakpoints.push(breakpoint);

                        // If the inferior is running, install breakpoints immediately
                        if let Some(inferior) = &mut self.inferior {
                            if let Err(e) = inferior.install_breakpoint(breakpoint) {
                                println!("Error setting breakpoint: {}", e);
                            }
                        }
                        println!("Set breakpoint {} at {:#x}", idx, breakpoint);
                    }
                }
            }
        }
    }

    fn handle_status(&mut self, status: Status) {
        match status {
            Status::Stopped(signal, rip) => {
                println!("Child stopped (signal {})", signal);
                let line_opt = self.debug_data.get_line_from_addr(rip);
                let line_desc = match line_opt {
                    Some(line) => line.to_string(),
                    None => "<unknown>".to_string(),
                };
                let func_opt = self.debug_data.get_function_from_addr(rip);
                let func_name = match func_opt {
                    Some(name) => name,
                    None => "<unknown>".to_string(),
                };
                println!("Stopped at {} ({})", func_name, line_desc);
            },
            Status::Exited(exit_code) => {
                println!("Child exited (status {})", exit_code);
                self.inferior = None;
            },
            Status::Signaled(signal) => {
                println!("Child terminated (signal {})", signal);
                self.inferior = None;
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
