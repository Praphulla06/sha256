fn main() {
    let message = String::from("RedBlockBlue");
    let msg_bits = parse_to_bits(&message);
    let msg_bits_length = msg_bits.len();

    println!("{}", msg_bits_length);
    let no_of_blocks = msg_bits_length / 512; // N
    println!("{}", no_of_blocks);

    let no_of_words = msg_bits_length / 32;
    println!("no of words: {}", no_of_words);

    println!("{:#?}", bits_to_u32(msg_bits, no_of_words));
    
}

fn parse_to_bits(message: &String) -> String{
    let mut msg_bits = String::new();
    for chr in message.as_bytes() {
        msg_bits.push_str(&format!("{:08b}", chr));
    }
    msg_bits.push('1');
    
    let msg_bit_length = msg_bits.len();
    
    let no_of_zeros = (((msg_bit_length / 512) + 1) * 512) - 64 - msg_bit_length;
    msg_bits.extend(std::iter::repeat('0').take(no_of_zeros));
    
    msg_bits.push_str(&format!("{:064b}", msg_bit_length - 1));
    msg_bits
}

fn bits_to_u32(msg_bits: String, no_of_words: usize) -> Vec<u32>{
    let sub_strings: Vec<u32> = msg_bits.as_bytes()
        .chunks(no_of_words)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    sub_strings
}