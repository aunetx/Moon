use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

const FILENAME: &str = "tables.moon";
const DEBUG: bool = true;

fn main() {
    // On récupère le contenu du fichier Moon
    let content = get_file();
    let _compute_program = get_transformed_program(content);
}

// Rend le contenu du fichier FILENAME (const)
fn get_file() -> String {
    let content = match open_file(FILENAME) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error : file {} does not exists ({})", FILENAME, error);
            exit(1)
        }
    };
    content
}

// Permet d'ouvrir un fichier donné
fn open_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_transformed_program(content: String) -> Vec<Vec<String>> {
    // On transforme le programme en un tableau de tuples
    let program: Vec<&str> = content.split('\n').collect();

    // On transforme "program" en un tableau de tableaux contenant ins, op1 (et op2)
    let mut compute_program: Vec<Vec<String>> = Vec::new();
    let mut line_number = 0;
    for line in program {
        line_number += 1;
        let str_line: String = line.to_string();
        let line = get_transformed_line(str_line, line_number);
        if DEBUG {
            if line.len() == 3 {
                println!("{} {} {}", line[0], line[1], line[2]);
            } else {
                println!("{} {}", line[0], line[1]);
            }
        }
        compute_program.push(line);
    }
    compute_program
}

// Nous permet d'obtenir un array contenant instruction, opérande 1 et opérande 2 pour une ligne donnée
fn get_transformed_line(line: String, line_number: i32) -> Vec<String> {
    // On enlève les caractères en trop et on remplace les lignes nulles par nll:nll
    let mut line = line.replace(" ", "").replace("\n", "");
    if line.len() == 0 {
        line = String::from("nll:nll");
    }

    // On split instruction / opérandes
    let splitted: Vec<&str> = line.split(':').collect();
    // Notre résultat :
    let mut trans_line: Vec<String> = Vec::new();

    // On vérifie qu'il n'y ait pas trop de séparateurs
    if splitted.len() != 2 || splitted[0].len() == 0 || splitted[1].len() == 0 {
        eprintln!("Error : incorrect syntax line {lnb}", lnb = line_number);
        exit(1);
    } else {
        // Instruction :
        trans_line.push(splitted[0].to_string());
        let splitted: Vec<&str> = splitted[1].split(",").collect();

        // Opérande 1 :
        if splitted[0].len() == 0 {
            eprintln!("Error : incorrect syntax line {lnb}", lnb = line_number);
            exit(1);
        };
        trans_line.push(splitted[0].to_string());

        // Opérande 2 :
        if splitted.len() == 2 {
            trans_line.push(splitted[1].to_string());
        }
        // On retourne le tableau
        return trans_line;
    }
}
