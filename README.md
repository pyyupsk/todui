# Todui - A Simple Todo CLI in Rust

Todui is a terminal-based todo list manager written in Rust. It provides an intuitive and efficient way to manage tasks directly from the command line, using simple keyboard shortcuts.

## Features

- Add, remove, and toggle completion of todos
- Assign priorities and tags to tasks
- Add notes to individual todos
- Filter tasks based on status
- Fully navigable with keyboard shortcuts

## Installation

### Prerequisites

Ensure you have `curl` or `wget` installed. If you don't have them, install them using your package manager.

### Quick Install (Linux/macOS)

For an easy installation on Linux or macOS, you can use the following command to download and run the installation script:

With `curl`:

```sh
bash -c "$(curl -sLo- https://raw.githubusercontent.com/pyyupsk/todui/refs/heads/main/install.sh)"
```

With `wget`:

```sh
bash -c "$(wget -qO- https://raw.githubusercontent.com/pyyupsk/todui/refs/heads/main/install.sh)"
```

This will automatically download and execute the installation script, setting up Todui for you.

### Build and Install (Alternative Method)

If you prefer to build from source, clone the repository and build the project using Cargo:

```sh
git clone https://github.com/pyyupsk/todui.git
cd todui
cargo install --path .
```

## Usage

Run Todui from your terminal:

```sh
todui
```

## Keyboard Shortcuts

```sh
q      - Quit application  
a      - Add todo  
j/↓    - Move selection down  
k/↑    - Move selection up  
Space  - Toggle completion  
d      - Delete selected todo  
p      - Cycle priority  
t      - Add/edit tags  
n      - Add/edit note  
Tab    - Cycle through filters  
?      - Toggle this help  

Press Esc to close help
```

## Contributing

Feel free to submit issues or pull requests to improve Todui!

## License

This project is licensed under the [Apache 2.0 License](LICENSE).
