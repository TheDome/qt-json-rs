use std::collections::HashMap;

/// A JSON Value is the Enum containing a Value. This makes it easy to perform match operations
/// against it.
#[derive(Debug)]
pub enum JsonValue {
    /// This encapsulates a RUST string.
    String(String),
    /// Since JS uses 64Bit floats, we can use them also
    Number(f64),
    /// Another JavaScript Object containing a Map of keys and values.
    Object(Object),
    /// A JavaScript Array containing a list of values.
    Array(Vec<JsonValue>),
    /// The explicit undefined type
    Undefined,
    /// A Bool
    Bool(bool),
    /// A NULL value
    Null,
}

/// A JavaScript Object (i.e. A Map of keys and values where keys are strings)
#[derive(Debug)]
pub struct Object {
    /// The number of elements in the object
    pub size: u32,
    /// All extracted values from the Object. This does not include JavaScript specific values
    /// like prototypes and functions.
    pub values: HashMap<String, JsonValue>,
}

/// A spacial value which will be located at the base of a [`QJSONDocument`](struct.QJSONDocument.html)
#[derive(Debug)]
pub enum JsonBaseValue {
    Object(Object),
    Array(Vec<JsonValue>),
}

/**
 * This is the base element of a JSON Document.
 *
 * A JSON Document can have either an Array or An Object as a Base
 */
#[derive(Debug)]
pub struct JsonBase {
    /**
     * The size of the overall Object (not needed in Rust)
     */
    pub size: u32,
    /**
     * The number of Elements, this base has.
     * (Self-explainatory for Object and Array)
     */
    pub elements: u32,
    /**
     * The value of this json
     */
    pub value: JsonBaseValue,
}
