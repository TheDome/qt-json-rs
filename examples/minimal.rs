
extern crate env_logger;

use qt_json_rs::QJSONDocument;

fn main() {
    env_logger::init();
    let json_data = b"qbjs\
    \x01\x00\x00\x00\
    \x10\x00\x00\x00\
    \x02\x00\x00\x00\
    \x0C\x00\x00\x00\
    \x4A\x01\x00\x00";

    let document = QJSONDocument::from_binary(json_data.to_vec()).unwrap();

    // Prints an Array with 10 as value
    println!("{:?}", document.base);
}