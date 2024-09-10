//use std::vec;
use rand::Rng;
fn main() {
    let char_bank: [char; 65] = [
    '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z','A', 'B', 'C', 'D', 
    'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '*', '@', '!' ];

    const FILLER: char = 'a';

    let mut pass_array: [char; 16]= [FILLER; 16];
    for x in 1..16 {
        let  c = rand::thread_rng().gen_range(0..=64);
        pass_array[x] = char_bank[c];
    }
    let password = String::from_iter(pass_array);
    print!("{}",password);
    
}
