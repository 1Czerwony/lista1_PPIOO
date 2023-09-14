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
    unsafe{*ponteiro+=1;}
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
    print!("\tFor:\n\t");
    for i in (1..=rand::thread_rng().gen_range(2..=10)).rev() {
        mult = mult * i;
        if i == 1{
            print!("{i}");
        } else {
            print!("{i} * ");
        }
    }
    println!(" = {mult}\n");

    // Repetição utilizando WHILE
    let mut i = rand::thread_rng().gen_range(2..=10);
    mult = 1;
    print!("\tWhile:\n\t");
    while i > 0 {
        mult = mult * i;
        if i == 1{
            print!("{i}");
        } else {
            print!("{i} * ");
        }
        i -= 1;
    }
    println!(" = {mult}");
}

fn passagem_var() {
    println!("\nPassagem de Variáveis:");
    let a = 7;
    let b = 8;
    let mut result = 0;
    mult_valor(a, b, result);
    println!("\tPassagem por valor: {a} * {b} = {result}  (variável não muda o valor)");
    mult_ref(a, b, &mut result);
    println!("\tPassagem por referência: {a} * {b} = {result}");
}

fn mult_valor(a: i32, b: i32, mut _result: i32) {
    _result = a * b;
}

fn mult_ref(a: i32, b: i32, result: &mut i32) {
    *result = a * b;
}


fn recursao() {
    println!("\nRecursão:");
    let n = 40;
    let mut i = 0;
    let mut v: &mut Vec<u32> = &mut vec![0; n+1];
    let result = fibonacci(n.try_into().unwrap(), &mut i, &mut v);
    println!("\ttermo {} de fibonacci = {}", n, result);
    println!("\tnúmero de recursões = {}", i);
}

fn fibonacci(n: u32, i: &mut i32, v: &mut Vec<u32>) -> u32 {
    *i += 1;
    let index: usize = n.try_into().unwrap();
    if v[index] == 0{ 
        if (n) <= 1 {
            v[index] = n;
            return n;
        } else {
            v[index] = fibonacci(n - 1, i, v) + fibonacci(n - 2, i, v);
            return v[index];
        }
    } else {
        return v[index];
    }
}