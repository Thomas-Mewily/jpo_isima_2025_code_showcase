pub const fn factorielle_recursive(n : u32) -> u32
{
    match n
    {
        0 | 1 => 1,
        _ => n * factorielle_recursive(n-1)
    }
}

pub const fn factorielle_recursive_avec_si(n : u32) -> u32
{
    if n == 0 || n == 1 { 1 } 
    else { n * factorielle_recursive(n-1) }
}

pub const fn factorielle_iterative(mut n : u32) -> u32
{
    let mut resultat = 1;
    while n != 0
    {
        resultat *= n;
        n -= 1;
    }
    resultat
}

pub fn factorielle_exemple()
{
    let mut v = Vec::new();

    for i in 0..10
    {
        println!("factorielle({: >2}) = {: >7} (recursif) {: <7} (iteratif)", i, factorielle_recursive(i), factorielle_iterative(i))
    }
    println!();
}

/* 
.\rust\target\debug\jpo.exe
*/