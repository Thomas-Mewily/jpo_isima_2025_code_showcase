pub fn code_cesar(code : &str, decalage : i32, alphabet : &[char]) -> String
{
    let mut resultat = String::new();
    let alphabet_len = alphabet.len() as isize;

    for lettre in code.chars()
    {
        match alphabet.iter().position(|e| *e == lettre)
        {
            Some(indice) => 
            {
                let nouvelle_indice = indice as isize + decalage as isize;
                // 
                let indice_boucle = isize::rem_euclid(nouvelle_indice, alphabet_len);
                
                // pas correct car le modulo '%' ne correspond pas à la définition d'Euclide
                // let indice_boucle = nouvelle_indice % alphabet_len + alphabet_len;

                // version correcte du modulo sans appeler `rem_euclid` si jamais le decalage est négatif :
                // let indice_boucle = (nouvelle_indice % alphabet_len + alphabet_len) % alphabet_len;
                resultat.push(alphabet[indice_boucle as usize]);
            }
            None => resultat.push(lettre),
        }
    }
    resultat
}

pub fn code_cesar_exemple() 
{
    let alphabet : Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    println!("alphabet : {:?} len = {}", alphabet, alphabet.len());
    println!();

    let message = "bonjour la jpo d'isima 2024";
    let decalage = 42;
    let message_code = code_cesar(&message, decalage, &alphabet);
    let message_decode = code_cesar(&message_code, -decalage, &alphabet);

    println!("original: {message}");
    println!("coder   : {message_code}");
    println!("decoder : {message_decode}");
    println!();
}
