# Learn Rust

Este projeto tem como objetivo aprender como utilizar a linguagem Rust e suas ferramentas. Fique a vontade para contribuir.

Conteúdo baseado na playlist [Aprenda Rust](https://www.youtube.com/playlist?list=PLjSf4DcGBdiGCNOrCoFgtj0KrUq1MRUME) do @rochacbruno.

## Instalação

Aqui estou utilizando WSL com Arch Linux, basta executar `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` que o Rust junto do Cargo será instalado. Para o seu sistema operacional, cheque a [documentação](https://www.rust-lang.org/pt-BR/learn/get-started) oficial do Rust para fazer o passo a passo.

## Iniciando um projeto

Para iniciarmos um projeto, podemos utilizar o Cargo. O cargo é o gerenciador de pacotes do Rust, ele baixa as dependências, compila os pacotes, os torna distribuíves e muito mais.

Vamos utilizar o comando `cargo new <nome-projeto>` para criarmos um novo projeto. Isto irá criar um diretório com o nome escolhido contendo a seguinte estrutura:

``` 
.
├── Cargo.toml
└── src
    └── main.rs
```

O arquivo `Cargo.toml` salva as configurações do projeto e é de extrema importância. Fazendo um paralelo, ele seria como o package.json ou o composer.json. Note também que foi criado um diretório chamado `src` contendo o arquivo `main.rs`, este arquivo já possuí a função main necessária para compilarmos o projeto.

Dentro do nosso workspace (diretório) criado, podemos utilizar o próprio Cargo para fazer o build do projeto, basta utilizar o comando `cargo build`. Elealém de compilar e gerar o binário necessário, faz checagens durante o processo. Ao executar o comando, note que é gerado um arquivo `Cargo.lock` e um diretório `target`. O Cargo oferece uma variedade de opções para serem utilizadas, como por exemplo formatar nosso código para seguir as convenções do Rust com o comando `cargo fmt`. Explore a documentação para mais opções.

## Variáveis e Constantes

No rust, criamos variáveis utilizando a palavra chave `let`. O rust não permite definirmos variáveis no escopo global da aplicação, faz-se então necessário declarar variáveis dentro de funções. Em linguagens compiladas, normalmente é necessário também deletar a váriavel alocada para não acontecer um memory leak, em alguns casos o próprio compilador faz isso com um garbage collector. No caso do Rust, é utilizado o RAII (Resource Acquisition Is Initialization) já implicito na linguagem, ou seja, após um escopo ser terminado a váriavel deixará de existir. 

```rust
fn main () {
    let total = 30;
    println!("Trabalhou {} horas", total);
}
```

Toda vez que uma variável é definida no rust, precisamos tipar ela. No exemplo acima não fizemos isso, então o compilador utiliza inferência, ou seja, através da sintaxe ou do contexto o compilador é capaz de definir o tipo da variável. Vamos tipar ela como um inteiro de 32 bits.

```rust
fn main () {
    let total: i32 = 30;
    println!("Trabalhou {} horas", total);
}
```

E se, após definir a variável e atribuir um valor, eu tentar substituir este valor?

```rust
fn main () {
    let total: i32 = 30;
    println!("Trabalho {} horas", total);
    total = 44;
    println!("Trabalho {} horas", total);
}
```

O compilador irá informar um erro de imutabilidade. O que isso significa? Toda variável em Rust é imutável, portanto ao atribuir um valor a uma variável, não podemos trocá-lo. A menos que ele seja definido como mutável com a palavra chave `mut`.

```rust
fn main () {
    // i32, inteiro de 32 bits, positivo ou negativo.
    let mut total: i32 = 30;
    println!("Trabalhou {} horas", total);
    total = 44;
    println!("Trabalhou {} horas", total);
}
```

Na maior parte das vezes é preferível que o código seja imutável para termos uma maior segurança, avalie o contexto e utilize o que for melhor.

Outra particularidade do rust é que ele possuí tipagem forte, ou seja, quando defino uma váriavel com valor inteiro e tento trocar para outro tipo, isto gerará um erro. Se a variável iniciar como um inteiro e eu tentar atribuir um valor numérico para ela, o compilador acusará um erro. Caso seja necessário trocar o tipo da variável, basta defini-lá novamente com a palavra chave `let`.

A partir da váriavel 'horas', vamos imprimir quantos segundos um trabalhador trabalhou. Neste caso, o valor sempre será fixo e, em casos como este, não é legal utilizarmos valores que podem variar, é aí que entram as constantes. Podem serem definidas no contexto principal da aplicação, dentro de uma função ou no contexto interior de uma função, basta utilizar a palavra chave `const`. Por padrão do rust, constantes utilizam Screaming Snake Case para definição do seu nome, ou seja, tudo em maiúsculo e separado por underline (_). Deve SEMPRE ter seu tipo definido.

```rust
// u32 equivale a unsigned integer, só irá receber números positivos de 32 bits.
const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos", total_em_segundos);
}

```

## Escopo Interno
```rust
fn main() {
    let total: i32 = 30;
    {
        let total = 44;
        println!("Trabalhou {} horas", total);
    }
    println!("Trabalhou {} horas", total);
}
```

Dentro de um escopo, como a fn main, podemos definir um outro escopo como interno, isto irá printar as duas variáveis. Lembrando que variáveis em Rust só existem no próprio escopo, portanto mesmo que o escopo externo ainda não tenha sido finalizado, fora do interno a variável definida NÃO pode ser utilizada. O inverso pode ocorrer, utilizar uma variável do escopo externo no escopo interno, somando um valor a ela por exemplo.

```rust
fn main() {
    let total = 30;
    {
        let total = total + 20;
        println!("Trabalhou {} horas", total);
    }
    println!("Trabalhou {} horas", total);
}
```

## Tipos Primitivos do Rust

Os tipos primitos do Rust são dividos em duas categorias. Vamos analisar os dois.

### Escalares

É onde iremos armazenar apenas um valor dentro de uma escala conhecida, permitindo uma comparação direta entre valores. São exemplos de tipos escalares:

- inteiro (int);
- flutuante (floating point / float);
- booleano (bool);
- caractere (char).

### Compostos

Servem para agregar multiplos valores. São exemplos de tipos compostos:

- Tupla (tuple) - Armazena vários valores de tipos diferentes `(5, true, 42.1, 'a')`;
- Matriz (array) - Armazena vários valores de tipos iguais.

#### Inteiros

| bits | signed | unsigned |
|------|--------|----------|
| 8    | i8     | u8       |
| 16   | i16    | u16      |
| 32   | i32    | u32      |
| 64   | i64    | u64      |
| 128  | i128   | u128     |
| arch | isize  | usize    |


- Signed: Quando utilizamos a notação signed (i) quer dizer que a váriavel irá suportar números inteiros positivos ou negativos. Tem um intervalo de `-128 até 127`.

- Unsigned: Quando utilizamos a notação unsigned (u) quer dizer que a variável irá suportar somente números inteiros positivos. Tem um intervalo de `0 até 255`.

Quando não definirmos o tipo da variável, o Rust irá assumir i32 como padrão.

OBS: A linha `arch` irá variar de acordo com a arquitetura em que o binário é executado, se for arquitetura 32bits ele será i32 ou u32, se for 64bits será i64 ou u64.

#### Tupla 

A tupla tem um tamanho fixo, portanto a partir de sua definição, o tamanho não pode ser alterado. Caso queira tipar os elementos da tupla, é necessário tipar os elementos separadamente.

```rust
fn main () {
    let numbers: (i32, i32, f64) = (1, 2, 3.5);
    println!("{:?}", numbers);
}
```

A tupla, assim como um array, possuí indexação. Para acessar os elementos da tupla, utilizamos o '.'.

```rust
fn main() {
    let numbers = (1, 2, 3.5);
    println!("{:?}", numbers.2);
}
```

Podemos também desestruturar a tupla através de pattern matching. 

```rust
fn main() {
    let numbers = (1, 2, 3);

    let (a, b, c) = numbers;
    
    // a = 1, b = 2, c = 3
    println!("{:?}", a);
}
```

Apesar do tamanho da tupla não poder ser alterado uma vez que seja definido, o mesmo não pode ser dito para seus valores. Se definirmos a tupla como mútavel, é possível alterar seu valor. Basta acessar o index desejado e substituir o valor. Não é possível alterar o valor para um tipo diferente do que foi definido. Lembrando que caso não seja tipado, o próprio rust irá tipar para nós.

```rust
fn main() {
    // tupla mútavel
   let mut numbers = (1, 2, 3);
    
    // substituimos o index 0 de valor 1 para valor 50
    numbers.0 = 50;

    println!("{:?}", numbers);
}
```

#### Arrays

Diferente da tupla, um array deve ter os elementos do mesmo tipo. Podemos tipar também, no caso do array passamos o tipo e também a quantidade de elementos (a indexação não é levada em conta, portanto o 0 não é contado).

```rust
fn main() {
    // array do tipo i32 com 3 elementos
    let numbers: [i32;3] = [1, 2, 3];
  
    // como em toda linguagem, acessamos o index através de colchetes.
    println!("{:?}", numbers[0]);
}
```

O array também pode ser definido como mútavel, fazendo com que possamos trocar os valores através do seu index, desde que seja do mesmo tipo definido na criação do array.

Podemos fatiar o array utlizando slice.

```rust
fn main() {
    let numbers = [1, 2, 3];

    // Utilizamos o & para pegar uma referência do array e depois fazemos o slice, do elemento 1 até o 2, isso irá printar o segundo elemento do array.
    println!("{:?}", &numbers[1..2]);
}
```

## Memory Awareness

Quando o programa começa a ser executado, é dado a esse programa acesso a três tipos de memórias: Static, Stack e Heap. Quando programos em Rust, precisamos ter exata noção exata de onde cada elemento criado será alocado, isso ajuda na performance da aplicação e também na legibilidade do código.

- Static: É alocada assim que o programa começa a ser executado. Tem um tamanho fixo e permanece por todo o ciclo de vida do programa, ao finalizar o programa o espaço é liberado. O binário do programa, variáveis estáticas e strings literais são alocadas no espaço de memória static. Variáveis armazenadas aqui possuem um acesso mais fácil.

```rust
// Informamos ao compilador que essa variável deve ir no espaço static.
static _Y: u32 = 13;

fn main() {

}
```

- Stack: Quando o programa é executado, ele calcula o tamanho necessário para alocar os recursos fixos e cria o bloco de memória Stack, fazendo com que seu tamanho seja dinâmico até o limite definido pelo compilador. Seus valores tem a mesma duração que a função, portanto ao retornar a função os recursos serão desalocados. Argumentos de funções e variáveis locais são alguns exemplos. Os recursos são alocados em pilha (como o nome já diz), portanto acessar recursos pode requerir performance. Caso haja uma tentativa de alocar recursos além do que foi definido para o espaço stack, teremos um erro 'stack overflow'.

```rust
fn main() {
    // Serão todas alocadas em Stack e deixarão de existir assim que main() terminar de ser executada.
    let x = 5;
    let y = 5.5;
    let z = true;
}
```

- Heap: Em alguns casos, os valores que vamos trabalhar podem variar. Quando consultamos um banco de dados, por exemplo, não sabemos exatamente quantos dados serão retornados. Nestes casos o espaço de memória utilizado será o 'Heap'. Tem tamanho dinâmico que irá variar de acordo com o limite do computador. Seu ciclo de vida é definido pelo programador ou pela linguagem e os recursos são desalocados manualmente, via Garbage Collector ou via RAII, no Rust é feito através de RAII. Valores que devem permanecer além de funções, valores compartilhados entre threads, valores grandes, qualquer valor que não pode ser definido durante a compilação será alocado em Heap. 

```rust
fn main() {
    // Deixará de existir quando main() terminar, mas pode ser alterado manualmete
    let users = get_users();
}
```

## Strings 

No Rust, temos algumas diferentes opções para utilizarmos strings. Cada uma delas funciona para casos específicos, levando em conta a alocação de memória.

#### String Slice

Quando definimos uma string diretamente com aspas duplas, estamos dizendo que aquela string é imútavel e será armazenada na memória STATIC da aplicação. Importante salientar que o valor será armazenado em STATIC, a váriavel deixará de existir assim que seu contexto for encerrado (uma função por exemplo).

```rust
fn main() {
    // Será armazenado no espaço STATIC da aplicação, não pode ser manipulado 
    let s = "Vinicius";
    
    println!("{s}");
}
```

##### String

Caso a string a ser utilizada deve ser alterada, ou não sabemos o tamanho exato dela, utilizamos o objeto String do Rust, que irá armazenar os dados na memória HEAP da aplicação, podendo aumentar seu tamanho. 

```rust
use std::io;

fn main() {
    // Será armazenado no espaço HEAP da aplicação e pode ser não manipulado
    let mut s = String::new();
    println!("Digite um texto");
   
    // Recupero o que foi digitado pelo user e utilizo uma referência mutável da variável s
    io::stdin().read_line(&mut s).expect("Erro ao ler o console");
    
    println!("Você digitou {s}");
}
```

Para exemplos do objeto String, cheque a (documentação do Rust)[https://doc.rust-lang.org/std/string/struct.String.html]
