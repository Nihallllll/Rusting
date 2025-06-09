use borsh::{BorshSerialize, BorshDeserialize};

// This function demonstrates serialization and deserialization using the Borsh crate.
///
/// It defines a custom struct, serializes an instance of it into a byte buffer
/// using Borsh, and then deserializes it back into a new struct instance.
/// Finally, it asserts that the original and deserialized instances are equal.
pub fn borsh_example() { // Changed function name to snake_case (borsh_example)
    // Define a struct that we want to serialize and deserialize.
    // #[derive(BorshSerialize, BorshDeserialize)] tells the Borsh macro
    // to automatically generate the necessary code for serialization and deserialization.
    // #[derive(Debug)] allows us to print the struct easily for debugging.
    // #[derive(PartialEq)] allows us to compare two instances of MyStruct using `==`.
    #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
    struct MyStruct {
        id: u64,
        data: String,
        v: Vec<u32>,
    }

    // Create an instance of our struct with some data.
    let original = MyStruct {
        id: 42,
        data: "Hello, Borsh!".into(), // ".into()" converts &str to String
        v: vec![1, 2, 3],
    };

    // --- Serialization ---
    // Create an empty mutable Vec<u8> which will serve as our buffer
    // to hold the serialized bytes.
    let mut buffer: Vec<u8> = Vec::new();

    // Serialize the `original` struct into the `buffer`.
    // `.unwrap()` is used here for simplicity in an example;
    // in real applications, you would handle the `Result` gracefully.
    original.serialize(&mut buffer).unwrap();
    println!("Serialized bytes: {:?}", buffer);
    // You can inspect `buffer` to see the binary representation.

    // --- Deserialization ---
    // Deserialize the bytes from the `buffer` back into a `MyStruct` instance.
    // `try_from_slice` takes a slice `&[u8]` as input.
    // `.unwrap()` is again for example simplicity.
    let deserialized = MyStruct::try_from_slice(&buffer).unwrap(); // Pass &buffer (a slice)

    // --- Verification ---
    // Assert that the deserialized struct is identical to the original struct.
    // This confirms that serialization and deserialization worked correctly.
    assert_eq!(original, deserialized);

    // Print the successfully deserialized struct.
    println!("Successfully serialized and deserialized: {:?}", deserialized);
}

