use std::{env, ffi::OsString};

fn basc_argument_parse() {
    let args: Vec<String> = env::args().collect();
    assert!(!args.is_empty());

    // skip the program name, it's usually the first argument
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args: {:?}", args);
}

fn basc_args_os() {
    let os_args: Vec<OsString> = env::args_os().collect();
    assert!(!os_args.is_empty());
    println!("os_args: {:?}", os_args);
}

fn main() {
    // Basic Argument Parsing
    basc_argument_parse();
    // Basic `args_os`
    basc_args_os();
}
