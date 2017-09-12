//
// main.rs
//


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        foreign_links {
            Fmt(::std::fmt::Error);
            Io(::std::io::Error) #[cfg(unix)];
        }
    }
}

use errors::*;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;


const MAX_WORKER: usize = 4;



fn open(path: &str) -> Result<File> {
    File::open(path).chain_err(|| format!("Can't open '{}'", path))
}

fn read(path: &str) -> Result<String> {
    let mut result = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut result)?;

    Ok(result)
}

fn create(path: &str) -> Result<File> {
    File::create(path).chain_err(|| format!("Can't write to '{}'", path))
}

fn write(path: &str, text: &str) -> Result<()> {
    let mut file = create(path)?;
    file.write_all(text.as_bytes());
    Ok(())
}


fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    use std::fs::File;
    /*
    open("dummyfile");

    let s: Result<String> = read("tretrete222");
    let path = "dummy";

    let f: Result<File> = File::open(path).chain_err(|| format!("Can't open '{}'", path));
*/
    // This operation will fail
    File::open("tretrete").chain_err(|| "unable to open tretrete file")?;

    Ok(())
}
