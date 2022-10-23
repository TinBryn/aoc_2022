pub fn input_file() -> Result<std::fs::File, std::io::Error> {
    std::path::Path::parent(file!().as_ref())
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "root directory")
        })
        // .map(std::path::Path::to_path_buf)
        .and_then(|path| {
            let mut path = path.to_path_buf();
            path.push("input.txt");
            std::fs::File::open(path)
        })
}

#[macro_export]
macro_rules! set_dir {
    () => {
        let path = std::path::Path::parent(file!().as_ref());

        let this_directory = std::path::Path::parent(file!().as_ref()).unwrap();
        std::env::set_current_dir(this_directory).unwrap();
    };
}

#[macro_export]
macro_rules! input {
    () => {
        std::path::Path::parent(file!().as_ref())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "root directory")
            })
            .and_then(|path| {
                let mut path = path.to_path_buf();
                path.push("input.txt");
                std::fs::File::open(path)
            })
    };
}
