/*
Objetivo: Praticar match, enums e entrada de dados básica.
O Desafio: Conversor de Temperaturas Robusto Crie um programa CLI que converta temperaturas entre Celsius, Fahrenheit e Kelvin.
Requisitos:
    Use um enum para representar as unidades de temperatura (ex: TemperatureUnit::Celsius).
    Implemente uma função que receba o valor e a unidade de origem, e retorne o valor convertido.
    Use match para lidar com a lógica de conversão.
    Extra: Tente tratar o erro se o usuário digitar algo que não seja número (use Result ou expect).
*/

use std::io;


 #[derive(Debug)]
enum UnidadesTemperatura{
    Celsius,
    Fahrenheit,
    Kelvin
}

fn parse_unit(input: &str) -> Option<UnidadesTemperatura>{
    match input.trim().to_lowercase().as_str(){
        "c" | "celsius" => Some(UnidadesTemperatura::Celsius),
        "f" | "fahrenheit" => Some(UnidadesTemperatura::Fahrenheit),
        "k" | "kelvin" => Some(UnidadesTemperatura::Kelvin),
        _ => None,
    }
}

fn ler_entrada(msg: &str) -> String{
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    input
}

fn conversor_temperaturas(temp: f64, origem: &UnidadesTemperatura, destino: &UnidadesTemperatura) -> f64{
    match(origem, destino){
        (UnidadesTemperatura::Celsius, UnidadesTemperatura::Fahrenheit) => (temp*1.8) + 32.0,
        (UnidadesTemperatura::Celsius, UnidadesTemperatura::Kelvin) => temp+273.0,
        (UnidadesTemperatura::Fahrenheit, UnidadesTemperatura::Celsius) => (temp-32.0) / 1.8,
        (UnidadesTemperatura::Fahrenheit, UnidadesTemperatura::Kelvin) => (temp-32.0) * (5.0/9.0) + 273.0,
        (UnidadesTemperatura::Kelvin, UnidadesTemperatura::Fahrenheit) => ((temp-273.0) * 1.8) + 32.0,
        (UnidadesTemperatura::Kelvin, UnidadesTemperatura::Celsius) => temp-273.0,
        _ => temp,
    }
}

pub fn main_primeiro() {

    let temperatura: f64 = loop{
        
        let input = ler_entrada("Digite uma temperatura.");
        match input.trim().parse(){
            Ok(num) => break num,
            Err(_) => println!("Erro: Digite um número valido!")
        }
    };

    let unidade_entrada = loop{
        let input = ler_entrada("Digite a unidade de origem (c, f, k).");
        match parse_unit(&input){
            Some(unit) => break unit,
            None => println!("Erro: Unidade invalida (c, f, k)!")
        }
    };

    let unidade_destino = loop{
        
        let input = ler_entrada("Digite a unidade de destino (c, f, k).");
        match parse_unit(&input){
            Some(unit) => break unit,
            None => println!("Erro: Unidade invalida (c, f, k)!")
        }
    };

    let resultado = conversor_temperaturas(temperatura, &unidade_entrada, &unidade_destino);
    println!("{}º {:?} => {}º {:?}", temperatura, unidade_entrada, resultado, unidade_destino);

}


