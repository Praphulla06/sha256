fn main() {
    let word = String::from("RedBlockBlue");
    let mut bits = String::new();
    for chr in word.as_bytes() {
        bits.push_str(&format!("{:08b}", chr));
    }
    bits.push('1');

    let word_bit_length = bits.len();
    let no_of_zeros = (((word_bit_length / 512) + 1) * 512) - 64 - word_bit_length;

    bits.extend(std::iter::repeat('0').take(no_of_zeros));

    bits.push_str(&format!("{:064b}", word_bit_length - 1));

    println!("{}", bits);
    println!("{}", bits.len());
    let N = (bits.len() % 512) + 1;
}
