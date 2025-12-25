fn main() {
    let message = String::from("RedBlockBlue");
    let msg_bits = parse_to_bits(&message);
    println!("{}", msg_bits.len());


    let sub_strings: Result<Vec<u32>, &str>  = msg_bits.as_bytes()
        .chunks(16)
        .map(|chunk| {
            let mut value: u32 = 0;
            for &b in chunk {
                value <<= 1;
                match b {
                    b'0' => {},
                    b'1' => value |= 1,
                    _ => return Err("invalid character in bit string."),
                }
            }
            Ok(value)
        })
        .collect();

    println!("{:#?}", sub_strings);
    
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
    
    msg_bits.push_str(&format!("{:064b}", msg_bit_length));
    msg_bits
}

fn bits_to_u32(msg_bits: String) {

}