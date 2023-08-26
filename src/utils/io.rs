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
