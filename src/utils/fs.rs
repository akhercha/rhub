use std::fs;
use std::path::Path;

/// Check if the path exists.
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Get the directory name from a path.
pub fn get_directory_name(path: &str) -> Result<String, std::io::Error> {
    // Convert the input path into a PathBuf.
    let p = if path == "." {
        std::env::current_dir()?
    } else {
        std::path::PathBuf::from(path)
    };
    p.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path does not have a directory name",
            )
        })
}

/// Count the number of files in a directory.
pub fn count_files_in_path(path: &str) -> usize {
    let count_result = (|| -> Result<usize, std::io::Error> {
        let mut count = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                count += 1;
            }
        }
        Ok(count)
    })();

    match count_result {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting files: {}", e);
            std::process::exit(1);
        }
    }
}
