use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

const FILENAME: &str = "tables.moon";

fn main() {
    // On récupère le contenu du fichier Moon
    let content = match open_file(FILENAME) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error : file {} does not exists ({})", FILENAME, error);
            exit(1)
        }
    };

    // On transforme le programme en un tableau de tuples
    let program: Vec<&str> = content.split('\n').collect();

    let mut line_number = 0;
    for line in program {
        line_number += 1;
        let str_line: String = line.to_string();
        let (ins, op1, op2) = get_transformed_line(str_line, line_number);
        println!(
            "Instruction : {i}\nOperand 1 : {o1}\nOperand 2 : {o2}",
            i = ins,
            o1 = op1,
            o2 = op2
        )
    }
}

// Permet d'ouvrir un fichier donné
fn open_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// Nous permet d'obtenir un tuple contenant instruction, opérande 1 et opérande 2 pour une ligne donnée (String)
fn get_transformed_line(line: String, line_number: i32) -> (String, String, String) {
    // On enlève les caractères en trop et on remplace les lignes nulles par nll:nll
    let mut line = line.replace(" ", "").replace("\n", "");
    if line.len() == 0 {
        line = String::from("nll:nll");
    }

    // On split instruction / opérandes
    let splitted: Vec<&str> = line.split(':').collect();

    // On vérifie qu'il n'y ait pas trop de séparateurs
    if splitted.len() != 2 || splitted[0].len() == 0 || splitted[1].len() == 0 {
        eprintln!("Error : incorrect syntax line {lnb}", lnb = line_number);
        exit(1);
    } else {
        // On distingue maintenant instruction et opérandes
        let instruction: String = splitted[0].to_string();
        let operands: Vec<&str> = splitted[1].split(",").collect();
        // On vérifie qu'op1 existe
        let operand1 = if operands[0].len() != 0 {
            String::from(operands[0])
        } else {
            eprintln!("Error : incorrect syntax line {lnb}", lnb = line_number);
            exit(1);
        };
        // On donne une valeur quomcumquest à op2
        let operand2 = if operands.len() == 2 {
            String::from(operands[1])
        } else {
            String::new()
        };
        // On retourne le tuple
        (instruction, operand1, operand2)
    }
}
