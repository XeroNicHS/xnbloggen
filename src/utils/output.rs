// src/utils/output.rs

// ANSI color codes
const RED: &str = "\x1b[1;31m";      // Bold red
const YELLOW: &str = "\x1b[1;33m";   // Bold yellow
const BLUE: &str = "\x1b[0;34m";     // Blue
const GREEN: &str = "\x1b[1;32m";    // Bold green
const CYAN: &str = "\x1b[0;36m";     // Cyan
const GRAY: &str = "\x1b[0;90m";     // Gray
const RESET: &str = "\x1b[0m";       // Reset

// Message level enum
#[derive(Debug)]
pub enum Level {
    Error,
    Warning,
    Info,
    Success,
    Step,
}

// Core output function
pub fn print_msg(level: Level, message: &str) {
    let (color, prefix) = match level {
        Level::Error => (RED, "ERROR"),
        Level::Warning => (YELLOW, "WARNING"),
        Level::Info => (BLUE, "INFO"),
        Level::Success => (GREEN, "SUCCESS"),
        Level::Step => (CYAN, "STEP"),
    };
    eprintln!("{}[{}]{} {}", color, prefix, RESET, message);
}

// Convenience functions for different message levels
pub fn error(msg: &str) {
    print_msg(Level::Error, msg);
}

pub fn warning(msg: &str) {
    print_msg(Level::Warning, msg);
}

pub fn info(msg: &str) {
    print_msg(Level::Info, msg);
}

pub fn success(msg: &str) {
    print_msg(Level::Success, msg);
}

pub fn step(msg: &str) {
    print_msg(Level::Step, msg);
}

// Path display function (gray colored)
pub fn print_path(path: &str) {
    eprintln!("  {}Path:{} {}", GRAY, RESET, path);
}

// Check mark for successful items (green)
pub fn print_check(msg: &str) {
    eprintln!("  {}✓{} {}", GREEN, RESET, msg);
}

// File item display (gray bullet point)
pub fn print_file(msg: &str) {
    eprintln!("    {}·{} {}", GRAY, RESET, msg);
}

pub fn logo() {
    println!(r#"
██╗  ██╗███╗  ██╗   
╚██╗██╔╝████╗ ██║   [ xnBlogGen v0.1.1 ]
 ╚███╔╝ ██╔██╗██║   Simple Static Blog Generator
 ██╔██╗ ██║╚████║   BLOG: https://xeronichs.github.io
██╔╝ ██╗██║ ╚███║   © 2026 XeroNicHS · MIT License
╚═╝  ╚═╝╚═╝  ╚══╝ 
"#);
}

pub fn help() {
    println!(r#"Usage: xnbloggen.exe <command> [options]
Commands:
  create          Create a new blog project
  new <title>     Create a new blog post with the given title
  build           Build the blog into static files (for deployment)
  server          Start a local HTTP server to preview the blog
  help            Show this help message

Options:
  create:
    --root <path>     Blog project root directory (default: current directory)

  new:
    --post            Create a blog post (default)
    --page            Create a static page (about, contact, etc.)
    --root <path>     Blog project root directory (default: current directory)

  build:
    --root <path>     Blog project root directory (default: current directory)

  server:
    --port <port>     Port for the local server (default: 8000)
    --root <path>     Blog project root directory (default: current directory)
"#);
}