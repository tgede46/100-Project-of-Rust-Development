use std::io;


fn main() {
    println!("BIM Calculator");
    println!("Entrer votre poids en kg: ");

    let weigth=match get_input_as_f64() {
        Some(values) => values,
        None => {
            println!("Entrée invalide pour le poids. Veuillez entrer un nombre.");
            return;
        }
    };

    println!("Entrer votre taille en m: ");
    let height=match get_input_as_f64() {
        Some(values) => values,
        None => {
            println!("Entrée invalide pour la taille. Veuillez entrer un nombre.");
            return;
        }
    };

    if height==0.0{
        println!("La taille ne peut pas être zéro.");
        return;
    }

    let bmi=calculate_bmi(weigth, height);
    println!("Votre BMI est: {:.2}", bmi);

    let category=classify_bmi(bmi);
    println!("Votre catégorie de BMI est: {}", category);
}

fn get_input_as_f64()-> Option<f64>{
    let  mut input=String::new();
    io::stdin().
        read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
        }

}


fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn classify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal weight"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obesity"
    }
}