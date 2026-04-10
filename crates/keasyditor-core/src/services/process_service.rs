/// Service for running external processes.
///
/// Encapsulates shell command execution behind a testable interface and
/// provides convenience methods for applying Klassy and Kvantum themes.
use std::io;
use std::process::Command;

/// The result of running an external process.
#[derive(Clone, Debug)]
pub struct ProcessResult {
    /// The exit code returned by the process.
    pub exit_code: i32,
    /// The standard output captured from the process.
    pub stdout: String,
    /// The standard error captured from the process.
    pub stderr: String,
}

impl ProcessResult {
    /// Whether the process exited successfully (exit code 0).
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}

impl std::fmt::Display for ProcessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProcessResult(exitCode={}, stdout={} chars, stderr={} chars)",
            self.exit_code,
            self.stdout.len(),
            self.stderr.len()
        )
    }
}

pub struct ProcessService;

impl ProcessService {
    pub fn new() -> Self {
        Self
    }

    /// Run an external executable with the given arguments.
    pub fn run(&self, executable: &str, args: &[&str]) -> io::Result<ProcessResult> {
        let output = Command::new(executable).args(args).output()?;
        Ok(ProcessResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    /// Check whether an executable is available on the system PATH.
    pub fn is_available(&self, executable: &str) -> bool {
        Command::new("which")
            .arg(executable)
            .output()
            .is_ok_and(|o| o.status.success())
    }

    /// Apply a Klassy preset by invoking `klassy-settings`.
    pub fn apply_klassy_preset(&self, preset_name: &str) -> io::Result<ProcessResult> {
        self.run("klassy-settings", &["--preset", preset_name])
    }

    /// Apply a Kvantum theme by invoking `kvantummanager`.
    pub fn apply_kvantum_theme(&self, theme_name: &str) -> io::Result<ProcessResult> {
        self.run("kvantummanager", &["--set", theme_name])
    }

    /// Reconfigure KWin so that Klassy decoration changes take effect.
    ///
    /// Tries `qdbus6` first, then falls back to `qdbus`.
    pub fn reconfigure_kwin(&self) -> io::Result<ProcessResult> {
        let args = &["org.kde.KWin", "/KWin", "reconfigure"];
        let result = self.run("qdbus6", args)?;
        if result.is_success() {
            return Ok(result);
        }
        self.run("qdbus", args)
    }
}

impl Default for ProcessService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_result_success() {
        let r = ProcessResult {
            exit_code: 0,
            stdout: "ok".to_string(),
            stderr: String::new(),
        };
        assert!(r.is_success());
    }

    #[test]
    fn process_result_failure() {
        let r = ProcessResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: "error".to_string(),
        };
        assert!(!r.is_success());
    }

    #[test]
    fn process_result_display() {
        let r = ProcessResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: "warn".to_string(),
        };
        let s = format!("{}", r);
        assert!(s.contains("exitCode=0"));
        assert!(s.contains("stdout=5"));
        assert!(s.contains("stderr=4"));
    }

    #[test]
    fn run_echo() {
        let svc = ProcessService::new();
        let result = svc.run("echo", &["hello"]).unwrap();
        assert!(result.is_success());
        assert_eq!(result.stdout.trim(), "hello");
    }

    #[test]
    fn run_nonexistent_command() {
        let svc = ProcessService::new();
        let result = svc.run("__nonexistent_command_12345__", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn is_available_echo() {
        let svc = ProcessService::new();
        assert!(svc.is_available("echo"));
    }

    #[test]
    fn is_available_nonexistent() {
        let svc = ProcessService::new();
        assert!(!svc.is_available("__nonexistent_command_12345__"));
    }
}
