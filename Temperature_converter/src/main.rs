use std::io;
fn main() {
    println!("conversion de température");
    println!("1 celcuis en fahrienheit");
    println!("2 fahrenheit en celcuis");
    println!("choisiez 1 ou 2");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");
    let choix:u32= match choix.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide");
            return;
        }
    };
    if choix == 1{
        celcuis_to_fahrenheit();
    }else if choix==2 {
        fahrenheit_to_celcuis();
    }else {
        println!("vous avez choisir une option invalide");
    }

}

fn celcuis_to_fahrenheit(){
    println!("Entrez la température en celcuis");
    let mut celcuis = String::new();
    io::stdin().read_line(&mut celcuis).expect("Erreur de lecture");
    let celcuis:f64 = match celcuis.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide");
            return;
        }
    };
    let fat =(celcuis*9.0/5.0)+32.0;
    println!("{}°C = {}°F",celcuis,fat);
}

fn fahrenheit_to_celcuis(){
    println!("Entrez la température en fahrenheit");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Erreur de lecture");
    let fahrenheit:f64 = match fahrenheit.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide");
            return;
        }
    };
    let cel=(fahrenheit-32.0)*5.0/9.0;
    println!("{}°F = {}°C",fahrenheit,cel);

}