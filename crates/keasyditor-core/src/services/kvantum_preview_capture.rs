/// Service that captures a PNG screenshot of the real `kvantumpreview`
/// application, so KEasyDitor can embed a pixel-accurate Kvantum render
/// alongside its simplified canvas mock.
///
/// Workflow:
///   1. Spawn `kvantumpreview` as a background child process.
///   2. Wait briefly for the Qt window to render.
///   3. Invoke `spectacle -a -b -n -o <path>` to capture the active window
///      (which is the freshly-spawned kvantumpreview) into a PNG file.
///   4. Kill the child process.
///
/// The caller is expected to have already applied the Kvantum theme they
/// want to preview via `kvantummanager --set <name>` — we don't touch the
/// active theme here.
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::constants;

/// How long to wait for `kvantumpreview` to open its window and finish
/// rendering before we screenshot it.
const RENDER_WAIT: Duration = Duration::from_millis(1500);

/// Name of the generated PNG inside `~/.cache/keasyditor/`.
const PREVIEW_FILE: &str = "kvantumpreview.png";

/// Outcome of a single capture call.
#[derive(Debug, Clone)]
pub struct KvantumPreviewCapture {
    /// Absolute path to the captured PNG.
    pub png_path: PathBuf,
    /// Raw PNG bytes — handy for Iced `image::Handle::from_memory` which
    /// reliably bypasses any path-based caching.
    pub png_bytes: Vec<u8>,
}

pub struct KvantumPreviewCaptureService;

impl KvantumPreviewCaptureService {
    pub fn new() -> Self {
        Self
    }

    /// Run the full capture pipeline. Blocks for `RENDER_WAIT` + spectacle
    /// runtime; intended to be called inside a `Task::perform` so the UI
    /// stays responsive.
    pub fn capture(&self) -> Result<KvantumPreviewCapture, String> {
        // Verify both external tools are on PATH up-front so we return a
        // meaningful error instead of a generic spawn failure later.
        for cmd in ["kvantumpreview", "spectacle"] {
            let available = Command::new("which")
                .arg(cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if !available {
                return Err(format!("{} not found on PATH", cmd));
            }
        }

        let cache_dir = constants::keasyditor_cache_dir();
        std::fs::create_dir_all(&cache_dir).map_err(|e| format!("create cache dir: {}", e))?;
        let png_path = cache_dir.join(PREVIEW_FILE);

        // Remove any stale file so we can detect failure (spectacle writing
        // nothing) vs. success (spectacle wrote a new file).
        let _ = std::fs::remove_file(&png_path);

        // 1. Spawn kvantumpreview. Inherit stdin/stdout/stderr so Qt output
        //    doesn't get captured (and potentially block on a full pipe).
        let mut child = Command::new("kvantumpreview")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| format!("spawn kvantumpreview: {}", e))?;

        // 2. Wait for it to render.
        thread::sleep(RENDER_WAIT);

        // 3. Capture the active window via spectacle.
        //    -a: active window, -b: background/no GUI, -n: no notification,
        //    -o: output file.
        let spectacle = Command::new("spectacle")
            .args([
                "-a",
                "-b",
                "-n",
                "-o",
                &png_path.to_string_lossy(),
            ])
            .output();

        // 4. Kill the kvantumpreview child regardless of spectacle outcome.
        let _ = child.kill();
        let _ = child.wait();

        // Interpret spectacle result.
        match spectacle {
            Ok(out) if out.status.success() => {}
            Ok(out) => {
                return Err(format!(
                    "spectacle exited with code {:?}: {}",
                    out.status.code(),
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
            Err(e) => return Err(format!("spectacle not runnable: {}", e)),
        }

        // Spectacle may return success without actually writing the file
        // (e.g. if the user cancelled a dialog in an older version). Verify.
        if !png_path.exists() {
            return Err("spectacle returned OK but no file was written".to_string());
        }

        let png_bytes = std::fs::read(&png_path)
            .map_err(|e| format!("read captured PNG: {}", e))?;

        Ok(KvantumPreviewCapture { png_path, png_bytes })
    }
}

impl Default for KvantumPreviewCaptureService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_wait_is_reasonable() {
        assert!(RENDER_WAIT.as_millis() >= 500);
        assert!(RENDER_WAIT.as_millis() <= 5000);
    }

    #[test]
    fn preview_file_name_is_png() {
        assert!(PREVIEW_FILE.ends_with(".png"));
    }
}
