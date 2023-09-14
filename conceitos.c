#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <locale.h>

// Exemplo do uso de ponteiros
void ponteiro(){
    printf("\nPonteiros:\n");
    int a = 10;
    int *b = &a;
    printf("\ta = %d, b = %d\n", a,*b);
    a = 20;
    printf("\ta = %d, b = %d\n", a,*b);
}

// Exemplo de Expressão Condicional
void condicional(){
    printf("\nCondicionais:\n");
    int n = rand() % 100;
    printf("\tn = %d\n",n);
    if (n%2 == 0)
        printf("\tn é par\n");
    else
        printf("\tn é impar\n");
}

// Exemplos de Laços de Repetição
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

// Função Recursiva para cálculo de Fibonacci
int fibonacci(int n, int *i) {
    *i = *i+1;
    if (n <= 1)
        return n;
    else
        return fibonacci(n - 1, i) + fibonacci(n - 2, i);
}

// Exemplo de Recursão
void recursao(){
    int i = 0;
    printf("\nRecursão:\n");
    printf("\ttermo 10 de fibonacci: %d\n", fibonacci(41, &i));
    printf("\tnúmero de recursões: %d\n", i);
}

// Funcão de Soma com Passagem por Valor
void somaValor(int a, int b, int result){
    result = a + b;
}

// Funcão de Soma com Passagem por Referência
void somaRef(int *a, int *b, int *result){
    *result = *a + *b;
}

// Exemplos de Passagem de Variáveis por Valor e Referência
void passagemVar(){
    printf("\nPassagem de variáveis:\n");
    int a = 5, b = 7, result = 0;
    printf("\ta = %d, b = %d, result = %d\n",a,b,result);
    printf("\tsoma(a,b,result)\n");
    somaValor(a,b,result);
    printf("\tpassagem por valor:\n\ta = %d, b = %d, result = %d\n",a,b,result);
    somaRef(&a,&b,&result);
    printf("\tpassagem por referencia:\n\ta = %d, b = %d, result = %d\n",a,b,result);
}

int main(){
    srand(time(NULL));
    setlocale(LC_ALL, "");

    ponteiro();
    condicional();
    lacos();
    recursao();
    passagemVar();

    return 0;
}