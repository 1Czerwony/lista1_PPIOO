use rand::Rng;

fn main() {
    ponteiro();
    condicionais();
    lacos();
    passagem_var();
    recursao();
}

fn ponteiro(){
    println!("\nPonteiro:");
    let mut a: i32 = rand::thread_rng().gen_range(0..=100);
    let ponteiro: *mut i32 = &mut a;
    println!("\tvalor de a : {a}");
    unsafe {
        println!("\tvalor de a apontado pelo ponteiro : {}", *ponteiro);
    }

    //incrementa o valor de a através do ponteiro
    println!("\tincrementando em 1 o valor de a através do ponteiro");
    unsafe{
        *ponteiro+=1;
    }
    println!("\tvalor de a : {a}");
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

fn mult_valor(a: i32, b: i32, mut _result: i32) {
    _result = a * b;
}

fn mult_ref(a: i32, b: i32, result: &mut i32) {
    *result = a * b;
}

fn passagem_var() {
    println!("\nPassagem de Variáveis:");
    let a = 7;
    let b = 8;
    let mut result = 0;
    mult_valor(a, b, result);
    println!("\tPassagem por valor:\n\t{a} * {b} = {result}");
    mult_ref(a, b, &mut result);
    println!("\tPassagem por referência:\n\t{a} * {b} = {result}");
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