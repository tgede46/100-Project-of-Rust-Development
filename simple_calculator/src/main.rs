use std::io;
fn main() {
    println!("Bienvenue dans la calculatrice simple");

    println!("Entrez le premier nombre:");
    let mut premier = String::new();
    io::stdin().read_line(&mut premier).expect("Erreur de lecture");
    let premier:f64 = match premier.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide");
            return;
        }
    };

    let mut deuxieme = String::new();
    io::stdin().read_line(&mut deuxieme).expect("Erreur de lecture");
    let deuxieme:f64 = match deuxieme.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide");
            return;
        }
    };

        println!("Entrez l'opération (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Erreur de lecture");
        let result = match operation.trim() {
        "+" => addition(premier, deuxieme),
        "-" => soubstraction(premier, deuxieme),
        "*" => multiplication(premier, deuxieme),
        "/" => division(premier, deuxieme),
        _ => {
            println!("Opération invalide");
            return;
        }
    };
    println!("Result {}", result);
}

fn addition(a: f64, b: f64) -> f64 {
    a + b
}
fn soubstraction(a:f64,b:f64)->f64{
    a-b
}
fn multiplication(a:f64,b:f64)->f64{
    a*b
}
fn division(a:f64,b:f64)->f64{
    a/a
}