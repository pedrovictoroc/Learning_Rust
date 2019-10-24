use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!!!");


    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

    //Transforma a String em numero de 32 bits
    //sem sinal, captura erro se não for um número
    let guess: u32 = guess.trim().parse()
                .expect("Please type a number!");

    //Gera números aleatórios em um range de 1 a 101
    let secret_number = rand::thread_rng().gen_range(1,101);

    //Utiliza das arms do match para comparar, um a um,
    //que padrão combina com o número e a entrada
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
