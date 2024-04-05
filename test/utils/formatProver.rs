use std::fs;
use std::str::FromStr;
use hex;

fn hex_string_to_byte_array(hex_string: &str) -> Vec<u8> {
    hex::decode(&hex_string[2..]).expect("Decoding failed")
}

fn main() {
    let toml_content = fs::read_to_string("circuits/Prover.toml").expect("Failed to read TOML file");

    // Extracting the values from the TOML content, assuming the TOML file structure is known and consistent
    let hashed_message_hex = toml_content.lines()
        .find(|line| line.starts_with("hashed_message"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("hashed_message not found");

    let public_key_x_hex = toml_content.lines()
        .find(|line| line.starts_with("publicKeyX"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("publicKeyX not found");

    let public_key_y_hex = toml_content.lines()
        .find(|line| line.starts_with("publicKeyY"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("publicKeyY not found");

    let signature_hex = toml_content.lines()
        .find(|line| line.starts_with("signature"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("signature not found");

    // Converting hex strings to byte arrays
    let hashed_message = hex_string_to_byte_array(&hashed_message_hex);
    let public_key_x = hex_string_to_byte_array(&public_key_x_hex);
    let public_key_y = hex_string_to_byte_array(&public_key_y_hex);
    let signature = hex_string_to_byte_array(&signature_hex);

    // Ensure the byte arrays are of the correct lengths
    assert_eq!(hashed_message.len(), 32);
    assert_eq!(public_key_x.len(), 32);
    assert_eq!(public_key_y.len(), 32);
    assert_eq!(signature.len(), 64);

    // Output the arrays in Rust syntax
    println!("hashed_message: {:?}", hashed_message);
    println!("pub_key_x: {:?}", public_key_x);
    println!("pub_key_y: {:?}", public_key_y);
    println!("signature: {:?}", signature);
}
