/*
Objetivo: Entender quando usar String vs &str, e como funcionam métodos (impl).
O Desafio: Sistema de Cadastro de Carros Crie um sistema simples para gerenciar uma lista de carros.

Requisitos:
    Crie uma struct Carro com campos como modelo (String), ano (u32) e fipe (f64).
    Crie uma struct Concessionaria que tenha um campo estoque que é um Vec<Carro>.
    Implemente métodos na Concessionaria (impl):
        adicionar_carro(&mut self, carro: Carro) -> Note o &mut self.
        listar_carros(&self) -> Note o &self imutável.
        buscar_por_ano(&self, ano: u32) -> Option<&Carro> -> Retorne uma referência ao carro, não uma cópia.

    Na main, crie alguns carros, adicione-os e tente buscar um.
*/

use std::io;

#[derive(Debug)]
struct Carro {
    modelo: String,
    ano: u32,
    fipe: f64,
}

#[derive(Debug)]
struct Concessionaria {
    estoque: Vec<Carro>
}

fn adicionar_carro(concessionaria: &mut Concessionaria, carro: Carro){ 
    concessionaria.estoque.push(carro); 
    concessionaria;
}

fn listar_carros(concessionaria: &Concessionaria){
    for i in 0..concessionaria.estoque.len(){
        let carro = &concessionaria.estoque[i];
        println!("{:?}", carro);
    }
}

fn busca_por_ano(concessionaria: &Concessionaria, ano: u32){
    for i in 0..concessionaria.estoque.len(){
        let carro = &concessionaria.estoque[i];
        if carro.ano == ano {
            println!("{:?}", carro);
        }
    }
}

fn ler_entrada(msg: &str) -> String{
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    input
}


pub fn main_segundo(){

    let mut concessionaria = Concessionaria{
        estoque: Vec::new(),
    };
    loop{
        println!("=== MENU ===");
        println!("1. ADICIONAR CARRO.");
        println!("2. LISTAR CARROS.");
        println!("3. BUSCA POR ANO.");
        println!("4. SAIR.");
        let input: i32 = ler_entrada("Digite uma opção.").trim().parse().expect("Erro: Não foi possivel alterar o tipo da variavel");

        match input {
            1 => {
                let modelo_novo: String = ler_entrada("Digite um modelo.");
                let modelo_novo = modelo_novo.trim().to_string();
                let ano_novo: String = ler_entrada("Digite um ano.");
                let ano_novo = ano_novo.trim().parse().expect("Erro: Não foi possivel alterar o tipo da variavel");
                let fipe_novo: String = ler_entrada("Digite o preço da tabela fipe.");
                let fipe_novo = fipe_novo.trim().parse().expect("Erro: Não foi possivel alterar o tipo da variavel");
                let carro_novo = Carro{
                    modelo: modelo_novo,
                    ano: ano_novo,
                    fipe: fipe_novo,
                };
                adicionar_carro(&mut concessionaria, carro_novo);
                println!("Carro adicionado com sucesso.");
                println!("=========");
            },
            2 => {
                listar_carros(& concessionaria);
                println!("Listagem completa.");
                println!("=========");
            },
            3 => {
                let ano: u32 = ler_entrada("Digite um ano.").trim().parse().expect("Erro: Não foi possivel alterar o tipo da variavel");
                busca_por_ano(& concessionaria, ano);
                println!("Listagem completa.");
                println!("=========");
            },
            4 => break,
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
        }
    }

}