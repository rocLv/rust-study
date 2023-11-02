use rand::Rng;

mod first_submodule {
    pub fn print_char() {
        for c in 'a'..'Z' {
            print!("{}", c);
        }
        println!();
    }
}

mod second_submodule {
    pub fn print_char() {
        for c in 'A'..'z' {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
   first_submodule::print_char();
   second_submodule::print_char();
}
