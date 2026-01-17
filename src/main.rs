use log_scanner::{extract_messages, present_output, read_file, save_output};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // This line can be used to execute the code without command line and to see an example
    //let path = "./example/example2.log";

    let log_file = read_file(path);
    let undesired_notes = read_file("./input/undesired_notes.txt");

    let messages = extract_messages(&log_file, &undesired_notes);
    present_output(&messages);
    save_output("test", &messages);
}