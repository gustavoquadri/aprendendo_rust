use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    println!("Adivinhe o numero!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("Numero aleatorio: {}", secret_number);

    loop {
        
        println!("Digite o seu palpite.");
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Falha ao ler entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Acertou!!!");
                break;
            }
        }
    }

    
}
