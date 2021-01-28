use std::env;
use std::path::*;


fn main() {
    //todo debug-mono-symfile - needs glib.h
    //todo profiler-events requires macro definitions, not sure if portable
    //todo sgen-bridge
    //todo verify.h - needs glib.h as well
    // let path = Path::new("Mono/include/");
    // let mut headers = Vec::new();
    // get_headers(path, &mut headers);
    //
    // for header in headers {
    //     let binding = bindgen::Builder::default()
    //         .clang_arg("-I Mono/include/mono-2.0/")
    //         .header(header.to_str().unwrap())
    //         .generate()
    //         .unwrap();
    //     binding.write_to_file(format!("src/{}.rs",header.file_name().unwrap().to_str().unwrap()));
    // }
}

fn get_headers(path: &Path, out: &mut Vec<Box<PathBuf>>) {
    if path.is_file() && path.extension().is_some() && path.extension().unwrap().eq("h") {
        out.push(Box::new(path.to_path_buf()));
        return;
    } else {
        if path.is_dir() {
            for p in path.read_dir().unwrap() {
                get_headers(p.unwrap().path().as_path(), out);
            }
        }
    }
}