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

```
fn main () {
  let total = 30;
  println!("Trabalhou {} horas", total);
}
```

Toda vez que uma variável é definida no rust, precisamos tipar ela. No exemplo acima não fizemos isso, então o compilador utiliza inferência, ou seja, através da sintaxe ou do contexto o compilador é capaz de definir o tipo da variável. Vamos tipar ela como um inteiro de 32 bits.

```
fn main () {
  let total: i32 = 30;
  println!("Trabalhou {} horas", total);
}
```


