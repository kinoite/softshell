# softshell (sfsh)

A simple, customizable shell built with Rust, featuring Lua for flexible configuration and dynamic modules. `sfsh` (Soft Shell) aims to provide a fast, extensible, and personalized command-line experience.

## ✨ Features

* **Rust Core:** Built with Rust for performance, safety, and reliability.
* **Lua Configuration:** Personalize almost every aspect of your shell, from the prompt to custom commands and keybindings, using simple Lua scripts.
* **Modular Design:** Extend `sfsh`'s capabilities by writing and integrating custom Lua modules.
* **Command History:** Persistent history of your executed commands for easy recall.
* **Rich & Customizable Prompt:** Create highly informative and visually appealing prompts using ANSI colors and Unicode glyphs.

### 🚀 Installation

To install `softshell`, ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run the following command:

```bash
cargo install softshell
```

This will install the sfsh executable to your Cargo bin directory (e.g., ~/.cargo/bin). Make sure this directory is in your system's PATH.
Note on Glyphs: For the full visual experience of advanced prompts (like the "Pinnacle of Prettiness" theme), it is highly recommended to use a Nerd Font in your terminal emulator.

### ⚙️ Configuration
sfsh looks for its main configuration file (config.lua) in the following order:
 * $XDG_CONFIG_HOME/sfsh/config.lua (e.g., ~/.config/sfsh/config.lua on Linux/macOS)
 * ~/.config/sfsh/config.lua (common fallback if XDG_CONFIG_HOME is not set)
 * ~/.sfshrc.lua
 * ./config.lua (in the current directory where sfsh is launched from - useful for development)
A good starting point for your configuration is to create one of these files.

### 📄 Sample Configuration (.sfshrc.lua)
You can find a simple example of a configuration file here (link to your GitHub repo's simple config).
This file defines:
 * modules_path: Where sfsh will look for custom Lua modules.
 * history_file: The path to your command history file.
 * get_prompt(): The function that generates your shell prompt.
 * Custom Lua functions and commands accessible from the shell.

### 📦 Lua Modules
sfsh's extensibility comes from its Lua module system. You can write your own Lua files and place them in the directory specified by modules_path in your config.lua.
For example, the Softhostname module (used to fetch the hostname reliably) is a separate Lua file:
``softhostname.lua``
 
### ▶️ Usage
Once softshell is installed, simply run sfsh from your terminal:
sfsh

You can then type commands, and if you have a mycommands table defined in your Lua config, you can call those functions directly (e.g., mycommands.hello()

### 🤝 Contributing
Contributions are always welcome! If you have ideas for features, bug reports, or want to contribute code, please feel free to open issues or submit pull requests on the GitHub repository.

### ⚖️ License
softshell is distributed under the terms of the Apache License, Version 2.0.