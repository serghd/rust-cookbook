use std::path::Path;

pub fn evaluate_path() {
    let path = Path::new(".");
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut path_new = path.join("a").join("b");
    path_new.push("c");
    path_new.push("myfile.tag.gz");
    path_new.set_file_name("package.tgz");
    match path_new.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s)
    }
}