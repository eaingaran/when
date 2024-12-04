# when

A cross-platform command-line tool to display the creation, modification, and access timestamps of executable files.

## Description

`when` helps you understand when a command was first added to your system and when it was last updated. It provides information about:

* **Date created:** When the command was first added to your computer.
* **Date modified:** When the command was last modified (potentially by developers before being added to your system).
* **Date updated:** When the command was last updated on your computer.
* **Date accessed:** When the command was last used or accessed.

## Installation

### From Source

1. Make sure you have Rust and Cargo installed.
2. Clone this repository: `git clone https://github.com/eaingaran/when.git`
3. Build the project: `cargo build --release`
4. The executable will be located at `target/release/when`

### (Optional) Add to PATH

To use `when` from any directory, add the `target/release` directory to your system's `PATH` environment variable.

## Usage

```bash
  when <command> [options]
```

**Arguments:**

* `<command>`: The name of the command (executable) to check.

**Options:**

* `-v, --verbose`: Display verbose output with a legend.
* `--json`: Output the information in JSON format.

**Examples:**

```bash
  when git
```
```bash
  when python -v
```
```bash
  when notepad --json
```

## Output

**Human-readable (default):**

```text
Command       : git
Path          : C:\Program Files\Git\cmd\git.exe
Updated   : Sunday, November 12, 2023 at 02:15 PM
Created       : Tuesday, December 4, 2024 at 02:25 PM
Accessed      : Tuesday, December 4, 2024 at 02:25 PM
```

**JSON:**

```json
{
  "command": "git",
  "path": "C:\\Program Files\\Git\\cmd\\git.exe",
  "created": "Tuesday, December 4, 2024 at 02:25 PM",
  "updated": "Sunday, November 12, 2023 at 02:15 PM",
  "accessed": "Tuesday, December 4, 2024 at 02:25 PM"
}
```

## Contributing

Contributions are welcome! Feel free to open issues to report bugs or suggest new features. If you'd like to contribute code, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.