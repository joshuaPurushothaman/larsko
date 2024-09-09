use std::fmt::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use larsko::LarskoAst;

fn read_source(path: &Path) -> LarskoAst {
    let display = path.display();

    let mut source_file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't read {}: {}", display, why), // TODO: cleanup with `?` lol
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match source_file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed

    LarskoAst {}
}

fn compile(src_code: LarskoAst) -> String {
    String::from("meeboo")
}

// TODO: clap!
fn main() {
    println!("Hello, world!");

    let src_code = read_source(Path::new("./lasko_sources/1.lasko"));
    let compiled = compile(src_code);

    println!("{}", compiled);
}
