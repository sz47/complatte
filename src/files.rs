use std::fs;
use std::path::Path;

pub fn print_hello() {
    println!("hello");
}

pub fn import_data(filename: String) {
    let file_exists = Path::new(&filename).exists();
    if file_exists {
        // deser
    } else {
        // parse and ser
    }
}
