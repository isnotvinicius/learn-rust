# Learn Rust

Este projeto tem como objetivo aprender como utilizar a linguagem Rust e suas ferramentas. Fique a vontade para contribuir.

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

Dentro do nosso workspace (diretório) criado, podemos utilizar o próprio Cargo para fazer o build do projeto, basta utilizar o comando `cargo build`. Ele além de compilar e gerar o binário necessário, faz checagens durante o processo. Ao executar o comando, note que é gerado um arquivo `Cargo.lock` e um diretório `target`. O Cargo oferece uma variedade de opções para serem utilizadas, como por exemplo formatar nosso código para seguir as convenções do Rust com o comando `cargo fmt`. Explore a documentação para mais opções.

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
|

- Signed: Quando utilizamos a notação signed (i) quer dizer que a váriavel irá suportar números inteiros positivos ou negativos. Tem um intervalo de `-128 até 127`.

- Unsigned: Quando utilizamos a notação unsigned (u) quer dizer que a variável irá suportar somente números inteiros positivos. Tem um intervalo de `0 até 255`.

Quando não definirmos o tipo da variável, o Rust irá assumir i32 como padrão.

OBS: A linha `arch` irá variar de acordo com a arquitetura em que o binário é executado, se for arquitetura 32bits ele será i32 ou u32, se for 64bits será i64 ou u64.