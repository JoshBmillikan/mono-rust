
#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{Path, PathBuf};

    #[test]
    fn it_works() {
        let env_path = env::var("MONO_INSTALL_DIR").expect("Could not find MONO_INSTALL_DIR");
        let path = Path::new(env_path.as_str());
        let mut headers = Vec::new();
        get_headers(path,&mut headers);
        for head in headers {
            println!("found header: {}",head.to_str().unwrap());
        }
        assert_eq!(2 + 2, 4);
    }
    fn get_headers(path: &Path, out: &mut Vec<Box<PathBuf>>) {
        if path.is_file() && path.extension().is_some() && path.extension().unwrap().eq("h") {
            out.push(Box::new(path.to_path_buf()));
            return;
        }
        else {
            if path.is_dir() {
                for p in path.read_dir().unwrap() {
                    get_headers(p.unwrap().path().as_path(), out);
                }
            }
        }
    }
}
