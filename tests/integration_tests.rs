use log_scanner::{extract_messages, read_file};

// Test if the extract_warnings_and_errors function can process the output of the read_file function.
#[test]
fn can_extract_warnings_and_errors() {
    let path: String = String::from("./example/example2.log");
    let log_file = read_file(&path);
    let undesired_notes = read_file("./input/undesired_notes.txt");
    let messages = extract_messages(&log_file, &undesired_notes);
    assert_eq!(
        messages,
        [
            "WARNING: Multiple lenghts detected. \r",
            "ERROR: The expected file xxx does not exist.\r",
            "NOTE: Variable Test was initialized with missing values.\r"
        ]
    );
}
