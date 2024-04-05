use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn hex_string_to_u8_array(hex_string: &str) -> Vec<u8> {
    let cleaned_hex_string = if hex_string.starts_with("0x") {
        &hex_string[2..]
    } else {
        hex_string
    };

    hex::decode(cleaned_hex_string).expect("Decoding failed")
}

fn main() {
    if let Err(e) = convert() {
        eprintln!("Error during conversion: {}", e);
        return;
    }

    let toml_content =
        fs::read_to_string("circuits/Prover.toml").expect("Failed to read TOML file");

    let hashed_message_hex = toml_content
        .lines()
        .find(|line| line.starts_with("hashed_message"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("hashed_message not found");

    let public_key_x_hex = toml_content
        .lines()
        .find(|line| line.starts_with("publicKeyX"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("publicKeyX not found");

    let public_key_y_hex = toml_content
        .lines()
        .find(|line| line.starts_with("publicKeyY"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("publicKeyY not found");

    let signature_hex = toml_content
        .lines()
        .find(|line| line.starts_with("signature"))
        .and_then(|line| line.split('=').nth(1))
        .map(|s| s.trim().replace("\"", ""))
        .expect("signature not found");

    // Convert hex strings to u8 arrays
    let hashed_message = hex_string_to_u8_array(&hashed_message_hex);
    let public_key_x = hex_string_to_u8_array(&public_key_x_hex);
    let public_key_y = hex_string_to_u8_array(&public_key_y_hex);
    let mut signature = hex_string_to_u8_array(&signature_hex);
    signature.pop();

    let hashed_message_str = hashed_message
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let public_key_x_str = public_key_x
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let public_key_y_str = public_key_y
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let signature_str = signature
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let new_content = format!(
        "hashed_message = [{}]\npub_key_x = [{}]\npub_key_y = [{}]\nsignature = [{}]\n",
        hashed_message_str, public_key_x_str, public_key_y_str, signature_str
    );

    let path = Path::new("circuits/Prover.toml");
    let mut file = File::create(&path).expect("Failed to create file");
    file.write_all(new_content.as_bytes())
        .expect("Failed to write to file");

    println!("File updated successfully.");
}

fn convert() -> io::Result<()> {
    let hash_path = Path::new("data/hash.txt");
    let publickeyx_path = Path::new("data/publicKeyX.txt");
    let publickeyy_path = Path::new("data/publicKeyY.txt");
    let signature_path = Path::new("data/signature.txt");

    let output_path = Path::new("circuits/Prover.toml");

    let mut output_file = File::create(&output_path)?;

    let hash_value = read_first_line(&hash_path)?;
    let publickeyx_value = read_first_line(&publickeyx_path)?;
    let publickeyy_value = read_first_line(&publickeyy_path)?;
    let signature_value = read_first_line(&signature_path)?;

    writeln!(output_file, "hashed_message = \"{}\"", hash_value)?;
    writeln!(output_file, "publicKeyX = \"{}\"", publickeyx_value)?;
    writeln!(output_file, "publicKeyY = \"{}\"", publickeyy_value)?;
    writeln!(output_file, "signature = \"{}\"", signature_value)?;

    Ok(())
}

fn read_first_line(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut line = String::new();
    buf_reader.read_line(&mut line)?;
    Ok(line.trim_end().to_string())
}
