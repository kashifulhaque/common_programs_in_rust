#![allow(dead_code)]

mod generate_random;

fn main() {
    let size: u8 = 32;
    let password: String = generate_random::generate_password(size);

    println!("Password: {}", password);
}
