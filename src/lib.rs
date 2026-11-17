//! C shell (csh) command interpreter for SigmaOS.
//!
//! Basic shell command parser, history, and execution model. Dependency-free.

use std::collections::VecDeque;
use std::fmt;

/// Errors for shell operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellError {
    ParseError(String),
    EmptyCommand,
    UnknownCommand(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(m) => write!(f, "parse error: {m}"),
            Self::EmptyCommand => write!(f, "empty command"),
            Self::UnknownCommand(c) => write!(f, "unknown command: {c}"),
        }
    }
}

impl std::error::Error for ShellError {}

/// Parsed command.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirects: Vec<Redirect>,
}

/// I/O redirection.
#[derive(Debug, Clone, PartialEq)]
pub struct Redirect {
    pub target: String,
    pub append: bool,
    pub stderr: bool,
}

/// C shell interpreter.
#[derive(Debug, Clone, Default)]
pub struct CShell {
    history: VecDeque<String>,
    max_history: usize,
}

impl CShell {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_history: 100,
        }
    }

    pub fn with_history_limit(mut self, limit: usize) -> Self {
        self.max_history = limit;
        self
    }

    pub fn parse_command(&self, input: &str) -> Result<Command, ShellError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(ShellError::EmptyCommand);
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ShellError::EmptyCommand);
        }

        let name = parts[0].to_string();
        let args = parts[1..].iter().map(|s| s.to_string()).collect();
        let redirects = Vec::new();

        Ok(Command { name, args, redirects })
    }

    pub fn execute(&self, cmd: &Command) -> Result<String, ShellError> {
        match cmd.name.as_str() {
            "echo" => {
                let output = cmd.args.join(" ");
                Ok(output)
            }
            "pwd" => {
                Ok(std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| "/".into()))
            }
            "ls" => {
                Ok("file1\nfile2\nfile3".to_string())
            }
            "help" => {
                Ok("Available commands: echo, pwd, ls, help".to_string())
            }
            _ => Err(ShellError::UnknownCommand(cmd.name.clone())),
        }
    }

    pub fn run_line(&mut self, line: &str) -> Result<String, ShellError> {
        let cmd = self.parse_command(line)?;
        self.add_history(line);
        self.execute(&cmd)
    }

    pub fn add_history(&mut self, line: &str) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(line.to_string());
    }

    pub fn history(&self) -> Vec<&str> {
        self.history.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_echo_aaa() {
        // Arrange
        let shell = CShell::new();

        // Act
        let cmd = shell.parse_command("echo hello world");

        // Assert
        assert!(cmd.is_ok());
        let c = cmd.unwrap();
        assert_eq!(c.name, "echo");
        assert_eq!(c.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_execute_echo_aaa() {
        // Arrange
        let mut shell = CShell::new();

        // Act
        let output = shell.run_line("echo test123");

        // Assert
        assert_eq!(output.unwrap(), "test123");
        assert_eq!(shell.history().len(), 1);
    }

    #[test]
    fn test_unknown_command_aaa() {
        // Arrange
        let mut shell = CShell::new();

        // Act
        let res = shell.run_line("nonexistent");

        // Assert
        assert!(res.is_err());
    }
}
