# qt-json-rs

A simple parser for the Internal QT Binary JSON data format.

This parser will transform the popular
[QTBinary JSON](https://doc.qt.io/qt-6.2/qbinaryjson.html#toBinaryData)
format into usable format for rust applications.

## Use

Simply provida a binary encoded JSON Array to the function and it will parse it into an
internal JSON structure:

```rust
use qt_json_rs::QJSONDocument;

fn main(){
        let json_data = b"qbjs\
    \x01\x00\x00\x00\
    \x10\x00\x00\x00\
    \x02\x00\x00\x00\
    \x0C\x00\x00\x00\
    \x4A\x01\x00\x00";

    let document = QJSONDocument::from_binary(json_data.to_vec()).unwrap();

    println!("{:?}", document);
}
```

## Disclaimer

This library has been widely created by looking at the QT source code and performing reverse
engineering.
There is a possibility that the code will not work with other Version of QT JSON documents.
Any help with this library is welcome.
