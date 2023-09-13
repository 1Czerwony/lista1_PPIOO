// use std::io;
use rand::Rng;

fn main() {
    condicionais();
}

fn condicionais() {
    println!("Condicionais:");
    let n = rand::thread_rng().gen_range(-10..=10);
    println!("\tnumero gerado: {n}");
    if n > 0 {
        println!("\to número gerado é positivo");
    }
    else if n < 0 {
        println!("\to número gerado é negativo");
    }
    else {
        println!("\to número gerado é zero");
    }
}