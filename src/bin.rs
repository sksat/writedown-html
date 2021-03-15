use std::fs;
use std::io::{self, Read};
use std::path::Path;

use clap::{App, Arg};
use writedown::{self, Render};
use writedown_html as html;

fn main() {
    let matches = App::new("wd-html")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("SRC").help("source file").index(1))
        .get_matches();

    let fname = matches.value_of("SRC").unwrap();

    let file = Path::new(fname);
    let mut file = io::BufReader::new(fs::File::open(file).unwrap());

    let mut src = String::new();
    let _ = file.read_to_string(&mut src).unwrap();

    let html = html::from_str(&src).unwrap();
    let out = html.render();

    print!("{}", out);
}
