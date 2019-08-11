use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

const FILENAME: &str = "tables.moon";
const DEBUG: bool = true;
const MAX_ITERATIONS: i32 = 10_000;

fn main() {
    // Preproc
    if DEBUG {
        println!("\n----------------------------------------------");
        println!("                    PREPROC                    \n");
    }
    let content = get_file();
    if DEBUG {
        println!("✓ file {:?} loaded", FILENAME);
    }
    let compute_program = get_transformed_program(content);
    if DEBUG {
        println!("✓ program read and transformed");
    }
    let flags = get_flags(&compute_program);
    if DEBUG {
        println!("✓ list of flags made")
    }

    // Runtime
    if DEBUG {
        println!("\n----------------------------------------------");
        println!("                    RUNTIME                    \n\n╖");
    }
    run_program(compute_program, flags);
    if DEBUG {
        println!("╜\n----------------------------------------------");
        println!(" Program {:?} finished without error ", FILENAME)
    }
}

//------------------ RUNTIME ------------------\\

// Memory management
#[path = "mem.rs"]
mod mem;

fn run_program(program: Vec<Vec<String>>, flags: (Vec<String>, Vec<i32>)) {
    let mut prog_line: usize = 0;
    let mut iteration = 0;
    let max_line = program.len();

    let mut memory: mem::Memory = mem::init_memory();

    loop {
        iteration += 1;
        if iteration == MAX_ITERATIONS {
            eprintln!("{} iterations, closing the process", iteration);
            exit(1)
        }
        let result = compute(&program[prog_line], &flags, prog_line, memory);
        prog_line = result.0;
        memory = result.1;
        if DEBUG {
            println!(
                "╟╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╎ it {}\n║",
                iteration
            );
        }

        if prog_line >= max_line {
            break;
        }
    }
}

fn compute(
    line: &Vec<String>,
    flags: &(Vec<String>, Vec<i32>),
    line_number: usize,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    if DEBUG {
        if line.len() == 2 {
            println!(
                "╠═╡ line {} ╞══╡ {} ⇢ {}",
                line_number, line[0], line[1]
            );
        } else {
            println!(
                "╠═╡ line {} ╞══╡ {} ⇢ {}  {}",
                line_number, line[0], line[1], line[2]
            );
        }
    }

    //      Matching instruction and executing corresponding function
    let op1 = line[1].clone();
    // Check wether operand 2 exists
    let (op2, one_op) = if line.len() == 3 {
        (line[2].clone(), false)
    } else {
        if line[0] == "ret"
            || line[0] == "flg"
            || line[0] == "gto"
            || line[0] == "nll"
            || line[0] == "prt"
        {
            ("".to_string(), true)
        } else {
            eprintln!("Error : missing second operand line {}", line_number);
            exit(1)
        }
    };
    match line[0].as_str() {
        // Two operands needed :
        "var" => instruction::var(line_number, op1, op2, memory),
        "set" => instruction::nll(line_number, memory),
        "add" => instruction::nll(line_number, memory),
        "sub" => instruction::nll(line_number, memory),
        "mul" => instruction::nll(line_number, memory),
        "div" => instruction::nll(line_number, memory),
        "rst" => instruction::nll(line_number, memory),
        "jmp" => instruction::nll(line_number, memory),
        "jne" => instruction::nll(line_number, memory),
        "ctp" => instruction::nll(line_number, memory),
        // One operand needed :
        "ret" => instruction::nll(line_number, memory),
        "flg" => instruction::nll(line_number, memory),
        "gto" => instruction::nll(line_number, memory),
        "nll" => instruction::nll(line_number, memory),
        "prt" => instruction::nll(line_number, memory),
        _ => {
            eprintln!(
                "Error : unexpected instruction {} line {}",
                line[0], line_number
            );
            exit(1)
        }
    }
}

//---------------- INSTRUCTIONS ---------------\\

// Instructions management
#[path = "instruction.rs"]
mod instruction;

//------------------ PREPROC ------------------\\

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
    let program: Vec<&str> = content.lines().collect();

    // On transforme "program" en un tableau de tableaux contenant ins, op1 (et op2)
    let mut compute_program: Vec<Vec<String>> = Vec::new();
    let mut line_number = 0;
    for line in program {
        line_number += 1;
        let str_line: String = line.to_string();
        let line = get_transformed_line(str_line, line_number);
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

fn get_flags(program: &Vec<Vec<String>>) -> (Vec<String>, Vec<i32>) {
    let mut flags_names: Vec<String> = Vec::new();
    let mut flags_locat: Vec<i32> = Vec::new();
    let mut lnb = 0;
    for line in program {
        if line[0] == "flg".to_string() {
            flags_names.push(line[0].clone());
            flags_locat.push(lnb);
        }
        lnb += 1;
    }
    (flags_names, flags_locat)
}
