/// Composition de fonction
/// https://fr.wikipedia.org/wiki/Composition_de_fonctions
/// G o F
pub fn composition<F, G, X, Y, Z>(g : G, f : F) -> impl Fn(X) -> Z
    where
    F : Fn(X) -> Y,
    G : Fn(Y) -> Z
{
    move |x| { g(f(x)) }
}

pub fn doubler(x : i32) -> i32 { x * 2 }
pub fn au_carre(x : i32) -> i32 { x * x }

pub fn composition_exemple()
{
    let x = 5;

    println!("x = {}", x);
    println!("doubler(x) = {}", doubler(x));
    println!("au_carre(x) = {}", au_carre(x));
    println!();

    let doubler_puis_carre = composition(au_carre, doubler);
    println!("doubler_puis_carre(x) = {}", doubler_puis_carre(x));

    let carre_puis_doubler = composition(doubler, au_carre);
    println!("carre_puis_doubler(x) = {}", carre_puis_doubler(x));

    println!();

    let doubler_puis_carre_puis_carre_puis_doubler = composition(carre_puis_doubler, doubler_puis_carre);
    println!("doubler_puis_carre_puis_carre_puis_doubler(x) = {}", doubler_puis_carre_puis_carre_puis_doubler(x));

    println!();
}