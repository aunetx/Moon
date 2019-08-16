// Types declarations
pub type Memory = (Integer, Float, Char, Str);
type Integer = (Vec<String>, Vec<i32>);
type Float = (Vec<String>, Vec<f64>);
type Char = (Vec<String>, Vec<char>);
type Str = (Vec<String>, Vec<String>);

// Init types
pub fn init_memory() -> Memory {
    (init_integers(), init_float(), init_char(), init_string())
}
fn init_integers() -> Integer {
    (Vec::new(), Vec::new())
}
fn init_float() -> Float {
    (Vec::new(), Vec::new())
}
fn init_char() -> Char {
    (Vec::new(), Vec::new())
}
fn init_string() -> Str {
    (Vec::new(), Vec::new())
}

// Get value
fn get_name_index(name: &String, array: &Vec<String>) -> usize {
    match array.binary_search(name) {
        Ok(index) => index,
        Err(error) => {
            eprintln!(
                "Error : variable {} not found in name array ({})",
                name, error
            );
            std::process::exit(1)
        }
    }
}
pub fn get_value_integer(name: &String, memory: Memory) -> i32 {
    let index = get_name_index(name, &(memory.0).0);
    (memory.0).1[index]
}
pub fn get_value_float(name: &String, memory: Memory) -> f64 {
    let index = get_name_index(name, &(memory.0).0);
    (memory.1).1[index]
}
pub fn get_value_char(name: &String, memory: Memory) -> char {
    let index = get_name_index(name, &(memory.0).0);
    (memory.2).1[index]
}
pub fn get_value_string(name: &String, memory: Memory) -> String {
    let index = get_name_index(name, &(memory.0).0);
    (memory.3).1[index].clone()
}
pub fn get_value_type(name: String, memory: Memory) -> ((i32, f64, char, String), &'static str) {
    let var_type = match search_variable(&name, &memory) {
        Ok(tp) => tp,
        Err(e) => {
            eprintln!("Internal error (ERR_TYPE_UNKNOWN)");
            std::process::exit(1);
        }
    };
    match var_type {
        "int" => (
            (get_value_integer(&name, memory), 0.0, '\x00', String::new()),
            "int",
        ),
        "flt" => (
            (0, get_value_float(&name, memory), '\x00', String::new()),
            "flt",
        ),
        "chr" => (
            (0, 0.0, get_value_char(&name, memory), String::new()),
            "chr",
        ),
        "str" => ((0, 0.0, '\x00', get_value_string(&name, memory)), "str"),
        _ => {
            eprintln!("Error : unknown type {}", var_type);
            std::process::exit(1)
        }
    }
}

pub fn get_plain_value(name: String, memory: Memory) -> ((i32, f64, char, String), &'static str) {
    match name.parse::<i32>() {
        Ok(value) => ((value, 0.0, '\x00', String::new()), "int"),
        Err(e) => match name.parse::<f64>() {
            Ok(value) => ((0, value, '\x00', String::new()), "int"),
            Err(e) => get_value_type(name, memory),
        },
    }
}

// Search existing variable and get type
fn variable_type_exists(name: &String, var_type: &str, memory: &Memory) -> Result<usize, usize> {
    if var_type == "int" {
        (memory.0).0.binary_search(name)
    } else if var_type == "flt" {
        (memory.1).0.binary_search(name)
    } else if var_type == "chr" {
        (memory.2).0.binary_search(name)
    } else if var_type == "str" {
        (memory.3).0.binary_search(name)
    } else {
        eprintln!("Internal error (ERR_TYPE_UNKNOWN)");
        std::process::exit(1);
    }
}

pub fn search_variable(name: &String, memory: &Memory) -> Result<&'static str, usize> {
    let res = variable_type_exists(name, "int", memory);
    match res {
        Ok(res) => Ok("int"),
        Err(res) => {
            let res = variable_type_exists(name, "flt", memory);
            match res {
                Ok(res) => Ok("flt"),
                Err(res) => {
                    let res = variable_type_exists(name, "chr", memory);
                    match res {
                        Ok(res) => Ok("chr"),
                        Err(res) => {
                            let res = variable_type_exists(name, "flt", memory);
                            match res {
                                Ok(res) => Ok("flt"),
                                Err(res) => Err(1),
                            }
                        }
                    }
                }
            }
        }
    }
}

// Create variables
pub fn create_integer(name: &String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.0).0.push(name.clone());
    (new_mem.0).1.push(0);
    new_mem
}
pub fn create_float(name: &String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.1).0.push(name.clone());
    (new_mem.1).1.push(0.0);
    new_mem
}
pub fn create_char(name: &String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.2).0.push(name.clone());
    (new_mem.2).1.push('\x00');
    new_mem
}
pub fn create_string(name: &String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.3).0.push(name.clone());
    (new_mem.3).1.push(String::new());
    new_mem
}

// Remove variable
pub fn remove_variable_with_type(name: &String, var_type: &str, memory: Memory) -> Memory {
    let mut memory_changed = memory;
    match var_type {
        "int" => {
            (memory_changed.0)
                .0
                .remove(get_name_index(name, &(memory_changed.0).0));
            (memory_changed.0)
                .1
                .remove(get_name_index(name, &(memory_changed.0).0));
        }
        "flt" => {
            (memory_changed.1)
                .0
                .remove(get_name_index(name, &(memory_changed.0).0));
            (memory_changed.1)
                .1
                .remove(get_name_index(name, &(memory_changed.0).0));
        }
        "chr" => {
            (memory_changed.2)
                .0
                .remove(get_name_index(name, &(memory_changed.0).0));
            (memory_changed.2)
                .1
                .remove(get_name_index(name, &(memory_changed.0).0));
        }
        "str" => {
            (memory_changed.3)
                .0
                .remove(get_name_index(name, &(memory_changed.0).0));
            (memory_changed.3)
                .1
                .remove(get_name_index(name, &(memory_changed.0).0));
        }
        _ => {
            eprintln!("Error : unknown type {}", var_type);
            std::process::exit(1)
        }
    };
    memory_changed
}

// Set variable value : delete the old value, and create a new one WITH NEW INDEX.
pub fn set_integer(name: &String, value: (i32, f64, char, std::string::String), memory: Memory) -> Memory {
    let mut new_mem = remove_variable_with_type(name, "int", memory);
    (new_mem.0).0.push(name.clone());
    (new_mem.0).1.push(value.0);
    new_mem
}
pub fn set_float(name: &String, value: (i32, f64, char, std::string::String), memory: Memory) -> Memory {
    let mut new_mem = remove_variable_with_type(name, "flt", memory);
    (new_mem.1).0.push(name.clone());
    (new_mem.1).1.push(value.1);
    new_mem
}
pub fn set_char(name: &String, value: (i32, f64, char, std::string::String), memory: Memory) -> Memory {
    let mut new_mem = remove_variable_with_type(name, "chr", memory);
    (new_mem.2).0.push(name.clone());
    (new_mem.2).1.push(value.2);
    new_mem
}
pub fn set_string(name: &String, value: (i32, f64, char, std::string::String), memory: Memory) -> Memory {
    let mut new_mem = remove_variable_with_type(name, "str", memory);
    (new_mem.3).0.push(name.clone());
    (new_mem.3).1.push(value.3);
    new_mem
}