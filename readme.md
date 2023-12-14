# Learn Rust

Este projeto tem como objetivo aprender como utilizar a linguagem Rust e suas ferramentas. Fique a vontade para contribuir.

## Instalação

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
