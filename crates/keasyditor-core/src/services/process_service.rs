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

    /// Apply a Klassy preset by invoking `klassy-settings`.
    pub fn apply_klassy_preset(&self, preset_name: &str) -> io::Result<ProcessResult> {
        self.run("klassy-settings", &["--preset", preset_name])
    }

    /// Apply a Kvantum theme by invoking `kvantummanager`.
    pub fn apply_kvantum_theme(&self, theme_name: &str) -> io::Result<ProcessResult> {
        self.run("kvantummanager", &["--set", theme_name])
    }

    /// Force-apply a Plasma color scheme by routing through a different
    /// intermediate scheme first, then switching to the target. This is
    /// the only reliable way to force running Qt apps to re-read a color
    /// scheme whose file contents were rewritten under the same name:
    /// `plasma-apply-colorscheme` is a no-op when the target name matches
    /// the currently-active scheme (it prints "already in use" and skips
    /// both the `KGlobalSettings.notifyChange` broadcast and any
    /// `kdeglobals` update). Routing through an intermediate scheme makes
    /// both calls real switches, so the signal actually goes out.
    ///
    /// Picks the first non-matching scheme from
    /// `["BreezeLight", "BreezeDark", "Breeze"]` as the intermediate —
    /// these ship with the base Plasma desktop and are effectively always
    /// present. If none of those are installed, falls back to a direct
    /// apply of `scheme_name` (best effort).
    pub fn apply_plasma_colorscheme_forced(
        &self,
        scheme_name: &str,
    ) -> io::Result<ProcessResult> {
        const FALLBACKS: &[&str] = &["BreezeLight", "BreezeDark", "Breeze"];
        for intermediate in FALLBACKS {
            if *intermediate == scheme_name {
                continue;
            }
            if let Ok(r) = self.run("plasma-apply-colorscheme", &[intermediate])
                && r.is_success()
            {
                // Intermediate switched successfully — now switch back.
                return self.run("plasma-apply-colorscheme", &[scheme_name]);
            }
        }
        // No intermediate worked; best-effort direct apply.
        self.run("plasma-apply-colorscheme", &[scheme_name])
    }

    /// Write `[General] ColorSchemeHash=<hash>` into `~/.config/kdeglobals`
    /// via `kwriteconfig6`. Must be called after a Plasma color-scheme file
    /// changes in place, so running Qt applications notice a new hash on
    /// their next refresh and actually re-read the scheme. `plasma-apply-
    /// colorscheme` does NOT update this field itself (only `ColorScheme=`),
    /// which is why we maintain it manually.
    pub fn write_kdeglobals_colorscheme_hash(&self, hash: &str) -> io::Result<ProcessResult> {
        self.run(
            "kwriteconfig6",
            &[
                "--file",
                "kdeglobals",
                "--group",
                "General",
                "--key",
                "ColorSchemeHash",
                hash,
            ],
        )
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

}
