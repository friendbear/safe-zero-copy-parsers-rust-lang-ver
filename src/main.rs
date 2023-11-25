pub struct ParsedData<'a> {
    pub header: u8,
    pub payload: &'a str,
}

impl ParsedData<'_> {
    pub fn parse(data: &[u8]) -> ParsedData {
        let header = data[0];
    
        let payload = std::str::from_utf8(&data[1..data.len()]).unwrap();
    
        ParsedData {
            header,
            payload,
        }
    }
}

fn get_data() -> Vec<u8> {
    const DATA: [u8; 5] = [255, b't', b'e', b's', b't'];
    DATA.to_vec() // Return dynamically allocated array (Vector)
}
#[test]
fn test_main_1() {
    // Simulate getting data from somewhere else (Ex: Socket) (Rust allows us to return a object)
    let buffer = get_data();

    // Parse buffer into ParsedData struct
    let parsed_data = ParsedData::parse(&buffer);

    // Print payload content
    println!("{}", parsed_data.payload);
}

fn main() {
    // Simulate getting data from somewhere else (Ex: Socket) (Rust allows us to return a object)
    let buffer = get_data(); // Make it mutable for testing purposes

    // Parse buffer into ParsedData struct
    let parsed_data = ParsedData::parse(&buffer);

    // Print payload content
    println!("Original: {}", parsed_data.payload);

    // Tamper with the original buffer
    let mut cloned_buffer = buffer.clone();
    cloned_buffer[1] = b'j';
    // Parse buffer into ParsedData struct
    let parsed_data = ParsedData::parse(&cloned_buffer);

    // Print payload content
    println!("Tempered: {}", parsed_data.payload);
}