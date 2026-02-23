use std::env;
use std ::fs::File;
use std::io::Read;

fn main() {
    let args:Vec<String> =env::args().collect();

    if args.len()!=2{
        println!("xUsage: word_counter <filename>");
        return;
    }

    let filename=&args[1];
    println!("Read file {}",filename);

    // lecture du ficher
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: Could not open file {}", filename);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err)=file.read_to_string(&mut contents) {
        println!("Error: Could not read file {}", err);
        return;
    }

//     contage des mots
    let word_count=count_words(&contents);
    println!("Word Count: {}", word_count);
}


fn count_words(text:&str)->usize{
    text.split_whitespace().count()
}