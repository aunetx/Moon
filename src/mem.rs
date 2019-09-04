// * Types declarations
pub type Memory = (Integer, Float, Char, Str);
type Integer = (Vec<String>, Vec<i32>);
type Float = (Vec<String>, Vec<f64>);
type Char = (Vec<String>, Vec<char>);
type Str = (Vec<String>, Vec<String>);

// * Init Memory type (with each type inited in order)
pub fn init_memory() -> Memory {
    let mem = (init_integers(), init_float(), init_char(), init_string());
    // Need to init system variable ('_res')
    create_integer(&"_res".to_string(), mem)
}
// * Init each type of variables : tuple containing an array of names, coupled with array of corresponding value
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

// * Get index of a given variable
// ? Variable need to exist
// TODO: Change error managing, prefer fallbacks better than quitting the process
fn get_name_index(name: &String, array: &Vec<String>) -> usize {
    // Need to sort the array before searching
    let mut array = array.clone();
    array.sort_unstable();
    // TODO: Implement standard search to be more efficient
    match array.binary_search(name) {
        Ok(index) => index,
        Err(_) => {
            eprintln!("Error : variable {:?} not found in name array", name);
            std::process::exit(1)
        }
    }
}

// * Get value of a given variable (type defined for each function)
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

// * Get value and type of a given variable
pub fn get_value_type(name: String, memory: Memory) -> ((i32, f64, char, String), &'static str) {
    let var_type = match search_variable(&name, &memory) {
        Ok(tp) => tp,
        Err(_) => {
            eprintln!("Error : cannot find variable {:?} in names array", name);
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

// * Return the value contained in a given string (i32, f64 or variable value)
pub fn get_plain_value(name: String, memory: Memory) -> ((i32, f64, char, String), &'static str) {
    match name.parse::<i32>() {
        Ok(value) => ((value, 0.0, '\x00', String::new()), "int"),
        Err(_) => match name.parse::<f64>() {
            Ok(value) => ((0, value, '\x00', String::new()), "int"),
            // FALLBACK : return value contained in variable named 'name'
            Err(_) => get_value_type(name, memory),
        },
    }
}

// * Verify that variable with given type exist and return its value
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
        eprintln!("Internal error (ERR_TYPE_UNKNOWN : {})", var_type);
        std::process::exit(1);
    }
}

// * Search for the value of a variable : return Result<Ok(VALUE), Err(e)>
pub fn search_variable(name: &String, memory: &Memory) -> Result<&'static str, usize> {
    let res = variable_type_exists(name, "int", memory);
    match res {
        Ok(_) => Ok("int"),
        Err(_) => {
            let res = variable_type_exists(name, "flt", memory);
            match res {
                Ok(_) => Ok("flt"),
                Err(_) => {
                    let res = variable_type_exists(name, "chr", memory);
                    match res {
                        Ok(_) => Ok("chr"),
                        Err(_) => {
                            let res = variable_type_exists(name, "str", memory);
                            match res {
                                Ok(_) => Ok("str"),
                                Err(_) => Err(1),
                            }
                        }
                    }
                }
            }
        }
    }
}

// * Create variables
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

// * Set variable value
pub fn set_integer(name: &String, value: i32, memory: Memory) -> Memory {
    let mut new_mem = memory.clone();
    let index = get_name_index(name, &(new_mem.0).0);
    std::mem::replace(&mut (new_mem.0).1[index], value);
    new_mem
}
pub fn set_float(name: &String, value: f64, memory: Memory) -> Memory {
    let mut new_mem = memory.clone();
    let index = get_name_index(name, &(new_mem.1).0);
    std::mem::replace(&mut (new_mem.1).1[index], value);
    new_mem
}
pub fn set_char(name: &String, value: char, memory: Memory) -> Memory {
    let mut new_mem = memory.clone();
    let index = get_name_index(name, &(new_mem.2).0);
    std::mem::replace(&mut (new_mem.2).1[index], value);
    new_mem
}
pub fn set_string(name: &String, value: String, memory: Memory) -> Memory {
    let mut new_mem = memory.clone();
    let index = get_name_index(name, &(new_mem.3).0);
    std::mem::replace(&mut (new_mem.3).1[index], value);
    new_mem
}

// * Remove variable with given name
pub fn remove_variable_with_type(name: &String, var_type: &str, memory: Memory) -> Memory {
    let mut memory_changed = memory;
    match var_type {
        "int" => {
            let index = get_name_index(name, &(memory_changed.0).0);
            (memory_changed.0).0.remove(index);
            (memory_changed.0).1.remove(index);
        }
        "flt" => {
            let index = get_name_index(name, &(memory_changed.1).0);
            (memory_changed.1).0.remove(index);
            (memory_changed.1).1.remove(index);
        }
        "chr" => {
            let index = get_name_index(name, &(memory_changed.2).0);
            (memory_changed.2).0.remove(index);
            (memory_changed.2).1.remove(index);
        }
        "str" => {
            let index = get_name_index(name, &(memory_changed.3).0);
            (memory_changed.3).0.remove(index);
            (memory_changed.3).1.remove(index);
        }
        _ => {
            eprintln!("Error : unknown type {}", var_type);
            std::process::exit(1)
        }
    };
    memory_changed
}

// ! DEBUG : print memory map
pub fn print_memory(mem_clone: Memory) {
    let mut ct = 0;
    print!("║\n╟╶ Memory map\n║ * Int : ");
    for var in (mem_clone.0).0 {
        print!("{} = {} ; ", var, (mem_clone.0).1[ct]);
        ct += 1;
    }
    let mut ct = 0;
    print!("\n║ * Float : ");
    for var in (mem_clone.1).0 {
        print!("{} = {} ; ", var, (mem_clone.1).1[ct]);
        ct += 1;
    }
    let mut ct = 0;
    print!("\n║ * Char : ");
    for var in (mem_clone.2).0 {
        print!("{} = {} ; ", var, (mem_clone.2).1[ct]);
        ct += 1;
    }
    let mut ct = 0;
    print!("\n║ * String : ");
    for var in (mem_clone.3).0 {
        print!("{} = {} ; ", var, (mem_clone.3).1[ct]);
        ct += 1;
    }
    println!();
}
