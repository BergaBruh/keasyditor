/// File I/O abstraction layer.
///
/// Provides a simple interface for file and directory operations.
use std::fs;
use std::io;
use std::path::Path;

/// File I/O service with methods that can be used for dependency injection.
#[derive(Clone, Debug, Default)]
pub struct FileService;

impl FileService {
    pub fn new() -> Self {
        Self
    }

    /// Read the entire contents of a file as a UTF-8 string.
    pub fn read_file(&self, path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    /// Write `content` to a file, creating parent directories as needed.
    pub fn write_file(&self, path: &str, content: &str) -> io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)
    }

    /// Return `true` if a file exists at `path`.
    pub fn file_exists(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// Return `true` if a directory exists at `path`.
    pub fn directory_exists(&self, path: &str) -> bool {
        Path::new(path).is_dir()
    }

    /// List the paths of files and directories inside `path`.
    pub fn list_directory(&self, path: &str) -> io::Result<Vec<String>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            entries.push(entry.path().to_string_lossy().into_owned());
        }
        Ok(entries)
    }

    /// Recursively copy a directory tree from `source` to `destination`.
    pub fn copy_directory(&self, source: &str, destination: &str) -> io::Result<()> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let src_path = entry.path();
            let file_name = entry.file_name();
            let dst_path = Path::new(destination).join(file_name);

            if src_path.is_dir() {
                self.copy_directory(
                    &src_path.to_string_lossy(),
                    &dst_path.to_string_lossy(),
                )?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    /// Create a directory (and parents) at `path`.
    pub fn create_directory(&self, path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn test_dir() -> String {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let p = temp_dir().join(format!("keasyditor_test_{}_{}", std::process::id(), id));
        let _ = fs::remove_dir_all(&p); // clean up any leftover
        p.to_string_lossy().into_owned()
    }

    #[test]
    fn read_write_file() {
        let dir = test_dir();
        let svc = FileService::new();
        let path = format!("{}/test.txt", dir);

        svc.write_file(&path, "hello world").unwrap();
        assert_eq!(svc.read_file(&path).unwrap(), "hello world");

        // Cleanup
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_creates_parents() {
        let dir = test_dir();
        let svc = FileService::new();
        let path = format!("{}/a/b/c/deep.txt", dir);

        svc.write_file(&path, "nested").unwrap();
        assert!(svc.file_exists(&path));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn file_exists_positive_negative() {
        let svc = FileService::new();
        assert!(svc.file_exists("/proc/self/exe")); // always exists on Linux
        assert!(!svc.file_exists("/nonexistent_path_12345"));
    }

    #[test]
    fn directory_exists_positive_negative() {
        let svc = FileService::new();
        assert!(svc.directory_exists("/tmp"));
        assert!(!svc.directory_exists("/nonexistent_dir_12345"));
    }

    #[test]
    fn list_directory_basic() {
        let dir = test_dir();
        let svc = FileService::new();

        svc.write_file(&format!("{}/a.txt", dir), "a").unwrap();
        svc.write_file(&format!("{}/b.txt", dir), "b").unwrap();

        let entries = svc.list_directory(&dir).unwrap();
        assert_eq!(entries.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_directory_nonexistent() {
        let svc = FileService::new();
        assert!(svc.list_directory("/nonexistent_dir_12345").is_err());
    }

    #[test]
    fn create_directory_nested() {
        let dir = test_dir();
        let svc = FileService::new();
        let nested = format!("{}/x/y/z", dir);

        svc.create_directory(&nested).unwrap();
        assert!(svc.directory_exists(&nested));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn copy_directory_basic() {
        let dir = test_dir();
        let svc = FileService::new();

        let src = format!("{}/src", dir);
        let dst = format!("{}/dst", dir);

        svc.write_file(&format!("{}/file.txt", src), "content").unwrap();
        svc.create_directory(&format!("{}/sub", src)).unwrap();
        svc.write_file(&format!("{}/sub/nested.txt", src), "nested").unwrap();

        svc.copy_directory(&src, &dst).unwrap();

        assert_eq!(svc.read_file(&format!("{}/file.txt", dst)).unwrap(), "content");
        assert_eq!(svc.read_file(&format!("{}/sub/nested.txt", dst)).unwrap(), "nested");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_nonexistent_file() {
        let svc = FileService::new();
        assert!(svc.read_file("/nonexistent_12345").is_err());
    }
}
