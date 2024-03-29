// Memory management
#[path = "mem.rs"]
pub mod mem;

// * var: name, type          CREATE A VAR GIVEN A TYPE
pub fn var(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    // Verify that name is not reserved
    let name = check_reserved_name(op1, line_number);
    let type_var = op2;
    // Shadowing the variable : remove ancient one
    let memory_changed = match mem::search_variable(&name, &memory) {
        Ok(ancient_type_var) => {
            println!("Shadowed variable {}", name);
            mem::remove_variable_with_type(&name, ancient_type_var, memory)
        }
        Err(_) => memory,
    };
    // Return the memory with added var
    let memory_changed = match type_var.as_str() {
        "int" => mem::create_integer(&name, memory_changed),
        "flt" => mem::create_float(&name, memory_changed),
        "chr" => mem::create_char(&name, memory_changed),
        "str" => mem::create_string(&name, memory_changed),
        _ => {
            eprintln!("Error : unknown type {:?} line {}", type_var, line_number);
            std::process::exit(1);
        }
    };
    (line_number + 1, memory_changed)
}

// * set: var, (var|value)    SET VAR TO VALUE
pub fn set(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    // Verify that var is not reserved
    let name = check_reserved_name(op1, line_number);
    // Get wanted value and type
    let (value, type_value) = mem::get_plain_value(op2, memory.clone());
    // Change variable value
    let memory_changed = match type_value {
        "int" => mem::set_integer(&name, value.0, memory),
        "flt" => mem::set_float(&name, value.1, memory),
        "chr" => mem::set_char(&name, value.2, memory),
        "str" => mem::set_string(&name, value.3, memory),
        _ => {
            eprintln!("Error : unknown type {:?} line {}", type_value, line_number);
            std::process::exit(1);
        }
    };
    (line_number + 1, memory_changed)
}

// * add: (var|value), (var|value)        ADD TWO OPERANDS => _res
pub fn add(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    let res = &String::from("_res");
    // Get wanted value and type
    let (value_op1, value_type_op1) = mem::get_plain_value(op1.clone(), memory.clone());
    let (value_op2, value_type_op2) = mem::get_plain_value(op2.clone(), memory.clone());
    // Check if same type
    let memory = if value_type_op1 == value_type_op2 {
        match value_type_op1 {
            "int" => mem::set_integer(res, value_op1.0 + value_op2.0, memory),
            "flt" => mem::set_float(res, value_op1.1 + value_op2.1, memory),
            _ => {
                eprintln!(
                    "Error : can't perform operation 'add' on type {} line {}",
                    value_type_op1, line_number
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "Error : type of {:?} is not the same as {:?} ({} != {}) line {}",
            op1, op2, value_type_op1, value_type_op2, line_number
        );
        std::process::exit(1);
    };
    (line_number + 1, memory)
}

// * sub: (var|value), (var|value)        SUBSTRACT TWO OPERANDS => _res
pub fn sub(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    let res = &String::from("_res");
    // Get wanted value and type
    let (value_op1, value_type_op1) = mem::get_plain_value(op1.clone(), memory.clone());
    let (value_op2, value_type_op2) = mem::get_plain_value(op2.clone(), memory.clone());
    // Check if same type
    let memory = if value_type_op1 == value_type_op2 {
        match value_type_op1 {
            "int" => mem::set_integer(res, value_op1.0 - value_op2.0, memory),
            "flt" => mem::set_float(res, value_op1.1 - value_op2.1, memory),
            _ => {
                eprintln!(
                    "Error : can't perform operation 'add' on type {} line {}",
                    value_type_op1, line_number
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "Error : type of {:?} is not the same as {:?} ({} != {}) line {}",
            op1, op2, value_type_op1, value_type_op2, line_number
        );
        std::process::exit(1);
    };
    (line_number + 1, memory)
}

// * mul: (var|value), (var|value)        MULTIPLY TWO OPERANDS => _res
pub fn mul(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    let res = &String::from("_res");
    // Get wanted value and type
    let (value_op1, value_type_op1) = mem::get_plain_value(op1.clone(), memory.clone());
    let (value_op2, value_type_op2) = mem::get_plain_value(op2.clone(), memory.clone());
    // Check if same type
    let memory = if value_type_op1 == value_type_op2 {
        match value_type_op1 {
            "int" => mem::set_integer(res, value_op1.0 * value_op2.0, memory),
            "flt" => mem::set_float(res, value_op1.1 * value_op2.1, memory),
            _ => {
                eprintln!(
                    "Error : can't perform operation 'add' on type {} line {}",
                    value_type_op1, line_number
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "Error : type of {:?} is not the same as {:?} ({} != {}) line {}",
            op1, op2, value_type_op1, value_type_op2, line_number
        );
        std::process::exit(1);
    };
    (line_number + 1, memory)
}

// * div: (var|value), (var|value)        DIVISE TWO OPERANDS => _res
pub fn div(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    let res = &String::from("_res");
    // Get wanted value and type
    let (value_op1, value_type_op1) = mem::get_plain_value(op1.clone(), memory.clone());
    let (value_op2, value_type_op2) = mem::get_plain_value(op2.clone(), memory.clone());
    // Check if same type
    let memory = if value_type_op1 == value_type_op2 {
        match value_type_op1 {
            "int" => mem::set_integer(res, value_op1.0 / value_op2.0, memory),
            "flt" => mem::set_float(res, value_op1.1 / value_op2.1, memory),
            _ => {
                eprintln!(
                    "Error : can't perform operation 'add' on type {} line {}",
                    value_type_op1, line_number
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "Error : type of {:?} is not the same as {:?} ({} != {}) line {}",
            op1, op2, value_type_op1, value_type_op2, line_number
        );
        std::process::exit(1);
    };
    (line_number + 1, memory)
}

// * rst: (var|value), (var|value)        DIVISE TWO OPERANDS AND RETURN THE REST => _res
pub fn rst(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    let res = &String::from("_res");
    // Get wanted value and type
    let (value_op1, value_type_op1) = mem::get_plain_value(op1.clone(), memory.clone());
    let (value_op2, value_type_op2) = mem::get_plain_value(op2.clone(), memory.clone());
    // Check if same type
    let memory = if value_type_op1 == value_type_op2 {
        match value_type_op1 {
            "int" => mem::set_integer(res, value_op1.0 % value_op2.0, memory),
            "flt" => mem::set_float(res, value_op1.1 % value_op2.1, memory),
            _ => {
                eprintln!(
                    "Error : can't perform operation 'add' on type {} line {}",
                    value_type_op1, line_number
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "Error : type of {:?} is not the same as {:?} ({} != {}) line {}",
            op1, op2, value_type_op1, value_type_op2, line_number
        );
        std::process::exit(1);
    };
    (line_number + 1, memory)
}

// * gto: flag                GO TO THE DESIRED FLAG OR VALUE
pub fn gto(
    line_number: usize,
    name: String,
    flags: &(Vec<String>, Vec<i32>),
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    let new_line = match flags.0.binary_search(&name) {
        Ok(index) => index,
        Err(_) => match name.parse::<usize>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!(
                    "Error : unrecognized flag or number {:?} line {}",
                    name, line_number
                );
                std::process::exit(1);
            }
        },
    };
    (new_line, memory)
}

// * nll: nll                 DO NOTHING
pub fn nll(line_number: usize, memory: mem::Memory) -> (usize, mem::Memory) {
    (line_number + 1, memory)
}

//------------------- UTILS -------------------\\

// * Return name if not reserved
fn check_reserved_name(name: String, line_number: usize) -> String {
    if name.chars().next() != Some('_') {
        name
    } else {
        eprintln!(
            "Error : cannot make change on var {} (reserved name) line {}",
            name, line_number
        );
        std::process::exit(1)
    }
}
