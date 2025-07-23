/* softshell
you may be wondering:
"why is the codebase so small?"
its because softshell is still in developement;
its still very small and its philosophy is to be small and light but configurable as possible,
hence the small codebase.
*/

use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
    fs,
    error::Error,
    sync::OnceLock,
};
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use mlua::{Lua, Result as LuaResult, Value, StdLib, LuaSerdeExt};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_CYAN: &str = "\x1b[36m";

static LUA_STATE: OnceLock<Lua> = OnceLock::new();

fn get_lua_state() -> &'static Lua {
    LUA_STATE.get_or_init(|| {
        println!("Initializing Lua state...");
        let lua = Lua::new();
        if let Err(e) = lua.load_std_libs(StdLib::ALL) {
            eprintln!("Error loading Lua standard libraries: {}", e);
        }
        lua
    })
}

const CONFIG_FILE_NAMES: &[&str] = &[
    "config.lua",
    ".config.lua",
    ".sfshrc.lua",
    ".sfshrc",
    ".sfsh.rc",
];

fn find_config_file() -> Option<PathBuf> {
    for &name in CONFIG_FILE_NAMES.iter() {
        let path = PathBuf::from(name);
        if path.exists() {
            println!("Found config file: {}", path.display());
            return Some(path);
        }
    }
    None
}

fn get_history_file_path(config_history_file: &str) -> PathBuf {
    let mut path = PathBuf::new();
    if config_history_file.starts_with("~/") {
        if let Some(home_dir) = env::home_dir() {
            path.push(home_dir);
            path.push(&config_history_file[2..]);
        } else {
            eprintln!("Warning: HOME environment variable not set. Using default history file in current directory.");
            path.push(".sfsh_history");
        }
    } else {
        path.push(config_history_file);
    }
    path
}

fn init_lua() -> LuaResult<()> {
    let _ = get_lua_state();
    println!("Lua initialized (or already was).");
    Ok(())
}

fn load_lua_config(config_file: &str) -> LuaResult<()> {
    let lua = get_lua_state();

    let config_content = fs::read_to_string(config_file)
        .map_err(|e| mlua::Error::external(format!("Failed to read config file {}: {}", config_file, e)))?;

    if let Err(e) = lua.load(&config_content).exec() {
        eprintln!("Error loading Lua config '{}': {}", config_file, e);
        eprintln!("Using default shell settings.");
        return Ok(());
    }

    let modules_path_lua: String = lua.globals().get("modules_path").unwrap_or_else(|_| String::from("./modules"));
    eprintln!("Setting modules path to: {}", modules_path_lua);
    
    let package_table: mlua::Table = lua.globals().get("package")?;
    let current_path: String = package_table.get("path")?;
    let new_package_path = format!("{}/?.lua;{}", modules_path_lua, current_path);
    package_table.set("path", new_package_path)?;

    println!("Loaded configuration from {}", config_file);
    Ok(())
}

fn execute_lua_command(args: &[&str]) -> LuaResult<bool> {
    let lua = get_lua_state();

    let full_name = args[0];
    let parts: Vec<&str> = full_name.split('.').collect();

    if parts.len() != 2 {
        eprintln!("Invalid Lua command format. Use 'module.function [args]'.");
        return Ok(false);
    }

    let module_name = parts[0];
    let func_name = parts[1];

    let require_fn: mlua::Function = lua.globals().get("require")?;
    let module_table: mlua::Table = require_fn.call(module_name)?;
    let func: mlua::Function = module_table.get(func_name)?;

    let lua_args: Vec<Value> = args[1..].iter().map(|&s| Value::String(lua.create_string(s).unwrap())).collect();

    let result = func.call::<Value>(lua_args)?;

    match result {
        Value::String(s) => {
            println!("{}", s.to_str()?);
        },
        Value::Nil => {},
        _ => {
            println!("Lua function returned a non-string value of type: {}", result.type_name());
        }
    }

    Ok(true)
}

fn execute_system_command(args: &[&str]) {
    let program = args[0];
    let cmd_args = &args[1..];

    let mut command = Command::new(program);
    command.args(cmd_args);

    match command.spawn() {
        Ok(mut child) => {
            let status = child.wait();
            if let Err(e) = status {
                eprintln!("sfsh: Failed to wait for command: {}", e);
            }
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("sfsh: Command not found: {}", program);
            } else {
                eprintln!("sfsh: Error executing command '{}': {}", program, e);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    init_lua()?;

    let config_file_path_option = find_config_file();

    if let Some(config_path) = config_file_path_option {
        let config_str = config_path.to_str().ok_or_else(|| {
            Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid config file path")) as Box<dyn Error>
        })?;
        load_lua_config(config_str)?;
    } else {
        println!("No configuration file found among {:?}", CONFIG_FILE_NAMES);
        println!("Using default shell settings (no Lua config loaded).");
    }

    let lua = get_lua_state();

    let history_file_config: String = lua.globals().get("history_file").unwrap_or_else(|_| String::from("~/.sfsh_history"));
    let history_file_path = get_history_file_path(&history_file_config);

    let mut rl = Editor::<(), DefaultHistory>::new()?; 

    if rl.load_history(&history_file_path).is_err() {
        eprintln!("No previous history found at {:?}", history_file_path);
    }

    println!("Sfsh (Rust) - C Core with Lua Config/Modules");

    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current working directory: {}", e),
    }

    loop {
        let prompt_string = match lua.globals().get::<mlua::Function>("get_prompt") {
            Ok(get_prompt_fn) => {
                match get_prompt_fn.call::<String>(()) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error calling Lua get_prompt: {}", e);
                        String::from("sfsh_rust_error> ") // Corrected
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Lua function 'get_prompt' not found: {}", e);
                String::from("sfsh_rust_fallback> ") // Corrected
            }
        };

        let readline = rl.readline(&prompt_string);

        match readline {
            Ok(line) => {
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed_line)?; 

                let args: Vec<&str> = trimmed_line.split_whitespace().collect();
                if args.is_empty() {
                    continue;
                }

                match args[0] {
                    "exit" => {
                        println!("Exiting Sfsh. Goodbye!");
                        break;
                    }
                    "cd" => {
                        if args.len() == 1 {
                            if let Some(home_dir) = env::home_dir() {
                                match env::set_current_dir(&home_dir) {
                                    Ok(_) => {
                                        if let Ok(new_cwd) = env::current_dir() {
                                            println!("Changed directory to: {}", new_cwd.display());
                                        }
                                    }
                                    Err(e) => eprintln!("sfsh: Failed to change to home directory: {}", e),
                                }
                            } else {
                                eprintln!("sfsh: Home directory not found. Please set the HOME environment variable.");
                            }
                        } else if args.len() == 2 {
                            let path = args[1];
                            match env::set_current_dir(path) {
                                Ok(_) => {
                                    if let Ok(new_cwd) = env::current_dir() {
                                        println!("Changed directory to: {}", new_cwd.display());
                                    }
                                }
                                Err(e) => eprintln!("sfsh: Failed to change directory to '{}': {}", path, e),
                            }
                        } else {
                             eprintln!("sfsh: cd: too many arguments");
                        }
                    }
                    cmd if cmd.contains('.') => {
                        if let Err(e) = execute_lua_command(&args) {
                            eprintln!("sfsh: Lua command error: {}", e);
                            execute_system_command(&args);
                        }
                    }
                    _ => {
                        execute_system_command(&args);
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("Exiting Sfsh. Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(&history_file_path).expect("Failed to save history.");
    Ok(())
}
