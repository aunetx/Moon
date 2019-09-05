use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

const FILENAME: &str = "tables.moon";
const DEBUG: bool = true;
const MAX_ITERATIONS: i32 = 1_0;

// ! NOW TRYING TO REDESIGN HOW MEMORY VARIABLE IS MANAGED
// !
// ! We will try to use references to original variable instead of shadowing it :
// ! This way, functions acting on mem will be epured, faster and less bugued
// !
// ! This is a deep answer to the problem of duplicated variables with instruction `var`

fn main() {
    if DEBUG {
        println!("\n----------------------------------------------");
        println!("                    PREPROC                    \n");
    }
    // catch i/o error
    let content = match open_file(FILENAME) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error : file {} does not exists ({})", FILENAME, error);
            return;
        }
    };
    if DEBUG {
        println!("✓ file {:?} loaded", FILENAME);
    }
    // catch parsing errors
    let compute_program = match get_transformed_program(content) {
        Ok(content) => content,
        Err(line) => {
            eprintln!("Error : incorrect syntax line {}", line);
            return;
        }
    };
    if DEBUG {
        println!("✓ program parsed");
    }
    let flags = get_flags(&compute_program);
    if DEBUG {
        println!("✓ list of flags made :");
        for num in 0..flags.0.len() {
            println!(" - line {} : {}", flags.1[num], flags.0[num])
        }
    }

    if DEBUG {
        println!("\n----------------------------------------------");
        println!("                    RUNTIME                    \n\n╖");
    }
    // catch runtime error : error already printed, now need to close and return error code
    match run_program(compute_program, flags) {
        Ok(()) => (),
        Err(err_code) => {
            eprintln!("Error code : {}", err_code);
            return;
        }
    };
    // end of the program : if we are here, everything went well
    if DEBUG {
        println!("╜\n----------------------------------------------");
        println!(" Program {:?} finished without error ", FILENAME)
    }
}

// !------------------ RUNTIME ------------------! \\

// Memory management
pub mod mem;

fn run_program(program: Vec<Vec<String>>, flags: (Vec<String>, Vec<i32>)) -> Result<(), i32> {
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
        let result = compute(&program[prog_line], &flags, prog_line, memory)?;
        prog_line = result.0;
        memory = result.1;
        if DEBUG {
            // ! DEBUG : print memory
            mem::print_memory(memory.clone());
            // ! DEBUG

            println!(
                "╟╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╎ it {} ✓\n║",
                iteration
            );
        }

        if prog_line >= max_line {
            return Ok(());
        }
    }
}

fn compute(
    line: &[String],
    flags: &(Vec<String>, Vec<i32>),
    line_number: usize,
    memory: mem::Memory,
) -> Result<(usize, mem::Memory), i32> {
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
    let (op2, _one_op) = if line.len() == 3 {
        (line[2].clone(), false)
    } else if line[0] == "ret"
        || line[0] == "flg"
        || line[0] == "gto"
        || line[0] == "nll"
        || line[0] == "prt"
    {
        ("".to_string(), true)
    } else {
        eprintln!("Error : missing second operand line {}", line_number);
        exit(1)
    };
    match line[0].as_str() {
        // Two operands needed :
        "var" => Ok(instruction::var(line_number, op1, op2, memory)),
        "set" => Ok(instruction::set(line_number, op1, op2, memory)),
        "add" => Ok(instruction::add(line_number, op1, op2, memory)),
        "mul" => Ok(instruction::mul(line_number, op1, op2, memory)),
        "sub" => Ok(instruction::sub(line_number, op1, op2, memory)),
        "div" => Ok(instruction::div(line_number, op1, op2, memory)),
        "rst" => Ok(instruction::rst(line_number, op1, op2, memory)),
        "jmp" => Ok(instruction::nll(line_number, memory)),
        "jne" => Ok(instruction::nll(line_number, memory)),
        "ctp" => Ok(instruction::nll(line_number, memory)),
        // One operand needed :
        "ret" => Ok(instruction::nll(line_number, memory)),
        "flg" => Ok(instruction::nll(line_number, memory)),
        "gto" => Ok(instruction::gto(line_number, op1, flags, memory)),
        "nll" => Ok(instruction::nll(line_number, memory)),
        "prt" => Ok(instruction::nll(line_number, memory)),
        _ => {
            eprintln!(
                "Error : unexpected instruction {} line {}",
                line[0], line_number
            );
            exit(1);
        }
    }
}

// !---------------- INSTRUCTIONS ---------------! \\

// Instructions management
pub mod instruction;

// !------------------ PREPROC ------------------! \\

// * Open a given file
fn open_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// * Give us back an the splitted program with an array of lines like : [[ins, op1, op2], [ins, op1, op2], [ins, op1, op2], ...]
fn get_transformed_program(content: String) -> Result<Vec<Vec<String>>, i32> {
    let program: Vec<&str> = content.lines().collect();
    let mut compute_program: Vec<Vec<String>> = Vec::new();
    let mut line_number = 0;
    for line in program {
        line_number += 1;
        let str_line: String = line.to_string();
        let line = get_transformed_line(str_line, line_number)?;
        compute_program.push(line);
    }
    Ok(compute_program)
}

// * Give us back an array of instructions like : [instruction, operand1, operand2] for a given program line
fn get_transformed_line(line: String, line_number: i32) -> Result<Vec<String>, i32> {
    // removing chars that we don't need and replacing unused lines with 'nll:nll'
    let mut line = line.replace(" ", "").replace("\n", "");
    if line.is_empty() {
        line = String::from("nll:nll");
    }
    // splitting instruction / operands
    let splitted: Vec<&str> = line.split(':').collect();

    let mut trans_line: Vec<String> = Vec::new();
    // verifying that there is not too much separators
    if splitted.len() != 2 || splitted[0].is_empty() || splitted[1].is_empty() {
        // Err! incorrect syntax line [return]
        Err(line_number)
    } else {
        // Instruction :
        trans_line.push(splitted[0].to_string());
        let splitted: Vec<&str> = splitted[1].split(',').collect();

        // Operand 1 :
        if splitted[0].is_empty() {
            // Err! incorrect syntax line [return]
            Err(line_number)
        } else {
            trans_line.push(splitted[0].to_string());

            // Operand 2 :
            if splitted.len() == 2 {
                trans_line.push(splitted[1].to_string());
            }
            Ok(trans_line)
        }
    }
}

fn get_flags(program: &Vec<Vec<String>>) -> (Vec<String>, Vec<i32>) {
    let mut flags_names: Vec<String> = Vec::new();
    let mut flags_locat: Vec<i32> = Vec::new();
    let mut lnb = 0;
    for line in program {
        if line[0] == "flg" {
            flags_names.push(line[1].clone());
            flags_locat.push(lnb);
        }
        lnb += 1;
    }
    (flags_names, flags_locat)
}
