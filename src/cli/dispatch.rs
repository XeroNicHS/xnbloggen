// src/cli/dispatch.rs

use thiserror::Error;

use crate::utils::output::{logo, help};
use crate::commands::{create_cmd, new_cmd, build_cmd, server_cmd};
use crate::commands::{create_cmd::CreateError, new_cmd::NewError, build_cmd::BuildError, server_cmd::ServerError};
use crate::commands::new_cmd::NewKind;

#[derive(Debug)]
pub enum Command {
    Create { root: String },
    New { title: String, kind: NewKind, root: String },
    Build { root: String },
    Server { root: String, port: u16 },
    Help,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    ParseError(#[from] ParseError),

    #[error(transparent)]
    CreateError(#[from] CreateError),

    #[error(transparent)]
    NewError(#[from] NewError),

    #[error(transparent)]
    BuildError(#[from] BuildError),

    #[error(transparent)]
    ServerError(#[from] ServerError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("No command specified")]
    NoCommand,

    #[error("Missing argument\n  Details: {0}")]
    MissingArgument(String),

    #[error("Invalid option\n  Details: {0}")]
    InvalidOption(String),

    #[error("Unknown command\n  Command: {0}")]
    UnknownCommand(String),
}

pub fn run(argv: Vec<String>) -> Result<(), CliError> {
    logo();

    let cmd = match parse(argv) {
        Ok(command) => command,
        Err(ParseError::NoCommand) => { help(); return Ok(()); },
        Err(e) => { return Err(e.into()); }
    };

    dispatch(cmd)?;

    Ok(())
}

fn dispatch(cmd: Command) -> Result<(), CliError> {
    match cmd {
        Command::Help => {
            help();
        }
        Command::Create { root } => {
            // Implementation for creating a new blog project
            create_cmd::run(&root)?;
        }
        Command::New { title, kind, root} => {
            // Implementation for creating a new blog post
            new_cmd::run(&title, kind, &root)?;
        }
        Command::Build { root } => {
            // Implementation for building the blog into static files
            build_cmd::run(&root)?;
        }
        Command::Server { port, root } => {
            // Implementation for starting the local HTTP server
            server_cmd::run(port, &root)?;
        }
    }
    Ok(())
}

fn parse(argv: Vec<String>) -> Result<Command, ParseError> {
    if argv.len() < 2 {
        // Show help message
        return Err(ParseError::NoCommand);
    }

    match argv[1].to_lowercase().as_str() {
        "create" => parse_create(&argv[2..]),
        "new" => parse_new(&argv[2..]),
        "build" => parse_build(&argv[2..]),
        "server" => parse_server(&argv[2..]),
        "help" => Ok(Command::Help),
        other => Err(ParseError::UnknownCommand(other.into())),
    }
}

fn parse_create(args: &[String]) -> Result<Command, ParseError> {
    let mut root = ".".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].to_lowercase().as_str() {
            "--root" => {
                i += 1;
                if i < args.len() {
                    root = args[i].clone();
                } else {
                    return Err(ParseError::MissingArgument("Expected value after --root".into()));
                }
            }
            other => return Err(ParseError::InvalidOption(format!("Unknown option for 'create': {}", other))),
        }
        i += 1;
    }

    Ok(Command::Create { root })
}

fn parse_new(args: &[String]) -> Result<Command, ParseError> {
    if args.is_empty() {
        return Err(ParseError::MissingArgument("Title is required for 'new' command".into()));
    }

    let title = args[0].clone();
    let mut root = ".".to_string();

    let mut post = false;
    let mut page = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].to_lowercase().as_str() {
            "--root" => {
                i += 1;
                if i < args.len() {
                    root = args[i].clone();
                } else {
                    return Err(ParseError::MissingArgument("Expected value after --root".into()));
                }
            }
            "--post" => {
                post = true;
            }
            "--page" => {
                page = true;
            }
            other => return Err(ParseError::InvalidOption(format!("Unknown option for 'new': {}", other))),
        }
        i += 1;
    }

    if post && page {
        return Err(ParseError::InvalidOption("Cannot specify both --post and --page".into()));
    }

    let kind = if page {
        NewKind::Page
    } else {
        NewKind::Post
    };

    Ok(Command::New { title, kind, root })
}

fn parse_build(args: &[String]) -> Result<Command, ParseError> {
    let mut root = ".".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].to_lowercase().as_str() {
            "--root" => {
                i += 1;
                if i < args.len() {
                    root = args[i].clone();
                } else {
                    return Err(ParseError::MissingArgument("Expected value after --root".into()));
                }
            }
            other => return Err(ParseError::InvalidOption(format!("Unknown option for 'build': {}", other))),
        }
        i += 1;
    }

    Ok(Command::Build { root })
}

fn parse_server(args: &[String]) -> Result<Command, ParseError> {
    let mut root = ".".to_string();
    let mut port: u16 = 8000;

    let mut i = 0;
    while i < args.len() {
        match args[i].to_lowercase().as_str() {
            "--root" => {
                i += 1;
                if i < args.len() {
                    root = args[i].clone();
                } else {
                    return Err(ParseError::MissingArgument("Expected value after --root".into()));
                }
            }
            "--port" => {
                i += 1;
                if i < args.len() {
                    port = args[i].parse().map_err(|_| ParseError::InvalidOption("Invalid port number".into()))?;
                } else {
                    return Err(ParseError::MissingArgument("Expected value after --port".into()));
                }
            }
            other => return Err(ParseError::InvalidOption(format!("Unknown option for 'server': {}", other))),
        }
        i += 1;
    }

    Ok(Command::Server { port, root })
}