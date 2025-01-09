#![allow(dead_code)]

mod code_cesar;
use code_cesar::*;

mod factorielle;
use factorielle::*;

mod somme_entier;
use somme_entier::*;

mod composition;
use composition::*;

fn main() 
{
    for _ in 0..30 { println!(); }

    somme_entier_exemple();
    factorielle_exemple();
    code_cesar_exemple();
    composition_exemple();
}
