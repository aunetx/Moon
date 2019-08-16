// Memory management
#[path = "mem.rs"]
mod mem;

// var: name, type          CREATE A VAR GIVEN A TYPE
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
            if ancient_type_var != type_var {
                mem::remove_variable_with_type(&name, ancient_type_var, memory)
            } else {
                memory
            }
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

// set: var, (var|value)    SET VAR TO VALUE
pub fn set(
    line_number: usize,
    op1: String,
    op2: String,
    memory: mem::Memory,
) -> (usize, mem::Memory) {
    // Verify that var is not reserved
    let name = check_reserved_name(op1, line_number);
    // Get wanted value and type
    let (value, type_value) = mem::get_value_type(op2, memory.clone());
    // Change variable value
    let memory_changed = match type_value {
        "int" => mem::create_integer(&name, memory),
        "flt" => mem::create_float(&name, memory),
        "chr" => mem::create_char(&name, memory),
        "str" => mem::create_string(&name, memory),
        _ => {
            eprintln!("Error : unknown type {:?} line {}", type_value, line_number);
            std::process::exit(1);
        }
    };
    (line_number + 1, memory_changed)
}

// nll: nll                 DO NOTHING
pub fn nll(line_number: usize, memory: mem::Memory) -> (usize, mem::Memory) {
    (line_number + 1, memory)
}

//------------------- UTILS -------------------\\

// Return name if not reserved
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
