// use std::io;
use rand::Rng;

fn main() {
    condicionais();
    lacos();
    recursao();
}

fn condicionais() {
    println!("\nCondicionais:");
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

fn lacos() {
    println!("\nLaços:");
    // Repetição utilizando FOR
    let mut mult = 1;
    for i in (1..=4).rev() {
        mult = mult * i;
        if i == 4{
            print!("\t4! = {i}");
        } else {
            print!(" * {i}");
        }
    }
    println!(" = {mult}");

    // Repetição utilizando WHILE
    let mut i = 10;
    mult = 1;
    while i > 0 {
        mult = mult * i;
        if i == 10{
            print!("\t10! = {i}");
        } else {
            print!(" * {i}");
        }
        i -= 1;
    }
    println!(" = {mult}");
}

fn recursao() {
    println!("\nRecursão:");
    let mut i = 0;
    let result = fibonacci(10, &mut i);
    println!("\ttermo 10 de fibonacci = {}", result);
    println!("\tnúmero de recursões = {}", i);
}

fn fibonacci(n: i32, i: &mut i32) -> i32 {
    *i += 1;
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1, i) + fibonacci(n - 2, i);
    }
}