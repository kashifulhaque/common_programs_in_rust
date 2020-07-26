use rand::Rng;

pub fn random_number() {
    let mut rng = rand::thread_rng();
    println!("\n** GENERATE RANDOM NUMBERS **\n");

    let num1: u8 = rng.gen();
    let num2: u16 = rng.gen();
    println!("num1 (Unsigned 8-bit integer): {}", num1);
    println!("num2 (Unsigned 16-bit integer): {}", num2);

    // Another way to generate random numbers
    println!("Random unsigned 32-bit integer: {}", rng.gen::<u32>());
    println!("Random signed 32-bit integer: {}", rng.gen::<i32>());
    println!("Random 32-bit floating-point value: {}", rng.gen::<f32>());
}
