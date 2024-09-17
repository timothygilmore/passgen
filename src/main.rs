//use std::vec;
use figleter::FIGfont;
use rand::Rng;

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let banner = standard_font.convert("*******gen");

    println!("{}", banner.unwrap());
    println!("Here is your password: \n\n");

    fn generate_standard() -> String {
        let char_bank: Vec<char> = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
            'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
            'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '*', '@', '!', '#'
        ];

        let mut pass_vec: Vec<char> = vec![];
        for _y in 0..16 {
            let c = rand::thread_rng().gen_range(0..=65);
            pass_vec.push(char_bank[c]);
        }
        let password = String::from_iter(pass_vec);
        password

    }
    println!("{}\n\n", generate_standard());
    println!("Do you want to keep a record of this password? (Yes)\n1. Yes\n2. New Standard Password\n3. Quit")
}
