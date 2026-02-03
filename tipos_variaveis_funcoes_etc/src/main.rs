fn main() {
    
    let x = 5;
    println!("valor da variavel x: {}", x);

    // x = 6;
    // println!("valor da variavel x: {}", x);

    // para se tornar mutavel
    // tem que usar o atributo "mut"
    let mut y = 5;
    println!("valor da variavel y: {}", y);

    y = 6;
    println!("valor da variavel y: {}", y);

    // constante são imutaveis e o tipo
    // deve ser declarado
    const NUMERO:i32 = 100;
    println!("valor da constante NUMERO: {}", NUMERO);

    let z = 20;
    println!("valor da variavel z: {}", z);
    // aqui vai ficar somente nesse
    // escopo, nao altera fora das {}
    {
        let z = z + 5;
        println!("valor da variavel z: {}", z);
    }
    // tem como declarar uma variavel nova
    // com o nome da antiga
    let z = z + 5;
    println!("valor da variavel z: {}", z);

    ////////////////////////////////////////////////
    ////////////////////////////////////////////////
    
    let _pode_ser_positivo_e_negativo: i32 = -42;
    let _so_positivo: u32 = 42;
    
    let _float_32:f32 = 3.14;
    let _float_64:f64 = 2.784654987; 

    let _positivo: bool = true;

    let _letra: char = 'A';

    ////////////////////////////////////
    
    let tuple: (&str, u32) = ("Rustando", 10);
    let (string, numero) = tuple;
    println!("string: {}, numero: {}", string, numero);
    println!("string: {}, numero: {}", tuple.0, tuple.1);

    ////////////////////////////////////
    
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("primeiro: {}, ultimo: {}", array[0], array[4]);

    let mut mut_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("primeiro: {}, ultimo: {}", array[0], array[4]);
    mut_array[0] = 5;
    mut_array[4] = 1;
    println!("primeiro: {}, ultimo: {}", mut_array[0], mut_array[4]);

    outra_funcao("legal");

    let numero:i32 = 5;
    println!("numero antes: {}", numero);
    let numero_depois: i32 = soma(numero);
    println!("numero depois: {}", numero_depois);

    loop_exemplo();

    loop_outro();
    // while é normal

    for_exemplo();
}


fn outra_funcao(x: &str){
    println!("outra funcao {}", x);
}

// funcao com retorno
fn soma(numero: i32) -> i32{
    numero + 1
    // ou
    // return numero + 1;
}

fn loop_exemplo(){
    let mut numero_teste = 0;
    println!("numero é: {}", numero_teste);
    loop {

        if numero_teste == 10{
            break;
        }
        numero_teste += 1;
        println!("numero é: {}", numero_teste);
    }
    println!("numero parou em: {}", numero_teste);
}

fn loop_outro(){

    let mut numero_teste = 10;
    println!("numero é: {}", numero_teste);
    'outro: loop {

        if numero_teste == 00{
            break;
        }
        numero_teste -= 1;
        println!("numero é: {}", numero_teste);
    }
    println!("numero parou em: {}", numero_teste);
}

fn for_exemplo(){

    let numeros: [i32; 10] = [0, 5, 7, 9, 10, 16, 18, 22, 26, 30];

    for numero in numeros.iter(){
        println!("numero: {}", numero)
    }

    for numero in 0..5{
        println!("numero: {}", numero)
    }
}