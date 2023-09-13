#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <locale.h>

void ponteiro(){
    printf("\nPonteiros:\n");
    int a = 10;
    int *b = &a;
    printf("\ta = %d, b = %d\n", a,*b);
    a = 20;
    printf("\ta = %d, b = %d\n", a,*b);
}

void condicional(){
    printf("\nCondicionais:\n");
    int n = rand() % 100;
    printf("\tn = %d\n",n);
    if (n%2 == 0)
        printf("\tn é par\n");
    else
        printf("\tn é impar\n");
}

void lacos(){
    printf("\nLaços:\n");

    /// Repetição utilizando FOR ///
    int n, soma = 0, i = 0;
    printf("\t0");
    for(i = 0; i<10; i++){
        n = rand() % 100;
        soma += n;
        printf(" + %d", n);
    }
    printf(" = %d\n", soma);

    // Repetição utilizando WHILE ///
    soma = 0, i = 0;
    printf("\t0");
    while(i < 5){
        n = rand() & 100;
        soma += n;
        printf(" + %d", n);
        i++;
    }
    printf(" = %d\n", soma);
}

int fibonacci(int n) {
    if (n <= 1)
        return n;
    else
        return fibonacci(n - 1) + fibonacci(n - 2);
}

void recursao(){
    printf("\nRecursão:\n");
    printf("\ttermo 10 de fibonacci: %d\n", fibonacci(10));
}

// void soma(int a, int b, int c){

// }

void passagemValor(){
    printf("\nPassagem por valor:\n");
    int a = 5;
    int b = 7;
    int c = 10;

}

int main(){
    srand(time(NULL));
    setlocale(LC_ALL, "");

    ponteiro();
    condicional();
    lacos();
    recursao();
    passagemValor();

    return 0;
}