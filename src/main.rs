//use std::vec;
use rand::Rng;
fn main() {
    let char_bank: [char; 55] = [
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

    for _x in 1..17 {
        let  c = rand::thread_rng().gen_range(0..=54);
       print!("{}",char_bank[c]) 
    }
    
}
