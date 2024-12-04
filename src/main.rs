use chrono::{DateTime, Local};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    json: bool,
}

#[derive(Serialize, Deserialize)]
struct Output {
    command: String,
    path: String,
    created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<String>,
    accessed: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend: Option<String>,
}

impl Output {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn to_string(&self, verbose: bool) -> String {
        let mut out_string = String::new();

        if verbose {
            if let Some(legend) = &self.legend {
                out_string.push_str("Legend\n");
                out_string.push_str(legend);
                out_string.push_str("\n");
            }

            out_string.push_str(&format!("Command       : {}\n", self.command));
        }

        out_string.push_str(&format!("Path          : {}\n", self.path));

        if let Some(modified) = &self.modified {
            out_string.push_str(&format!("Updated   : {}\n", modified));
        }

        out_string.push_str(&format!("Created       : {}\n", self.created));

        if let Some(updated) = &self.updated {
            out_string.push_str(&format!("Modified  : {}\n", updated));
        }

        out_string.push_str(&format!("Accessed      : {}\n", self.accessed));

        out_string
    }
}

#[derive(Serialize, Deserialize)]
struct OutputError {
    command: String,
    path: String,
    error: String,
}

impl OutputError {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn to_string(&self) -> String {
        format!(
            "Error: could not get the metadata for command: {} at path: {}. Error: {}",
            self.command, self.path, self.error
        )
    }
}

fn main() {
    let datetime_format: &str = "%A, %B %d, %Y at %I:%M %p";

    let args = Args::parse();

    let command_path = if cfg!(windows) {
        let output = Command::new("where").arg(&args.command).output().unwrap();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        let output = Command::new("which").arg(&args.command).output().unwrap();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    if !command_path.is_empty() {
        let path: &Path = Path::new(&command_path);
        match fs::metadata(path) {
            Ok(metadata) => {
                let created: DateTime<Local> = metadata.created().unwrap().into();
                let modified: DateTime<Local> = metadata.modified().unwrap().into();
                let accessed: DateTime<Local> = metadata.accessed().unwrap().into();

                let output: Output = Output {
                    command: args.command,
                    path: command_path,
                    created: created.format(datetime_format).to_string(),
                    modified: if created < modified {
                        Some(modified.format(datetime_format).to_string())
                    } else {
                        None
                    },
                    updated: if created > modified {
                        Some(modified.format(datetime_format).to_string())
                    } else {
                        None
                    },
                    accessed: accessed.format(datetime_format).to_string(),
                    legend: if args.verbose {
                        Some(String::from("Updated: This is when the command was likely last modified by its developers before being added to your computer.\n\
                                          Created: This is when the command was first added to your computer.\n\
                                          Modified: This is when the command was last changed or updated on your computer.\n\
                                          Accessed: This is when the command was last accessed or executed on your computer.\n"))
                    } else {
                        None
                    },
                };

                if args.json {
                    println!("{}", output.to_json());
                } else {
                    println!("{}", output.to_string(args.verbose));
                }
            }
            Err(error) => {
                if args.verbose || args.json {
                    let output_error: OutputError = OutputError {
                        command: args.command,
                        path: command_path,
                        error: error.to_string(),
                    };

                    if args.json {
                        println!("{}", output_error.to_json());
                    } else {
                        println!("{}", output_error.to_string());
                    }
                }
            }
        }
    }
}
