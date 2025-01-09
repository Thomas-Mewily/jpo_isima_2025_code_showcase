
pub fn somme_entier_avec_etapes(n : i64) -> i64
{
    let mut somme = 0;
    print!("0");

    for valeur in 1..=n
    {
        somme += valeur;
        print!(" + {}", valeur);
    }

    println!(" = {}", somme);
    somme
}

pub fn somme_entier_v1(n : i64) -> i64
{
    let mut somme = 0;

    for valeur in 1..=n
    {
        somme += valeur;
    }

    somme
}

pub fn somme_entier_v2(n : i64) -> i64 { (1..=n).sum() }

pub fn somme_entier_v3(n : i64) -> i64 { n*(n+1)/2 }


pub fn somme_entier_exemple()
{
    somme_entier_avec_etapes(7);
    println!();

    for n in 0..10
    {
        let v1 = somme_entier_v1(n);
        let v2 = somme_entier_v2(n);
        let v3 = somme_entier_v3(n);
        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
        println!("somme_entier({}) = {}", n, v1);
    }

    let dix_millions = 10_000_000;
    println!("La somme des entier de 0 à {} est de {}", dix_millions, somme_entier_v1(dix_millions));

    println!();
}