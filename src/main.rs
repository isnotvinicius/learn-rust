use std::io;

fn main() {
    let mut s = String::new();
    println!("Digite um texto:");

    io::stdin()
        .read_line(&mut s)
        .expect("Erro ao ler o console");

    println!("Você digitou {s}");
}
