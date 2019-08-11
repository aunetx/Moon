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
fn get_name_index(name: String, array: &Vec<String>) -> usize {
    match array.binary_search(&name) {
        Ok(index) => index,
        Err(error) => {
            eprintln!("Variable {} not found in name array ({})", name, error);
            std::process::exit(1)
        }
    }
}
pub fn get_value_integer(name: String, memory: &Memory) -> i32 {
    let index = get_name_index(name, &(memory.0).0);
    (memory.0).1[index]
}
pub fn get_value_float(name: String, memory: &Memory) -> f64 {
    let index = get_name_index(name, &(memory.0).0);
    (memory.1).1[index]
}
pub fn get_value_char(name: String, memory: &Memory) -> char {
    let index = get_name_index(name, &(memory.0).0);
    (memory.2).1[index]
}
pub fn get_value_string(name: String, memory: &Memory) -> String {
    let index = get_name_index(name, &(memory.0).0);
    (memory.3).1[index].clone()
}

// Create variables
pub fn create_integer(name: String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.0).0.push(name);
    (new_mem.0).1.push(0);
    new_mem
}
pub fn create_float(name: String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.1).0.push(name);
    (new_mem.1).1.push(0.0);
    new_mem
}
pub fn create_char(name: String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.2).0.push(name);
    (new_mem.2).1.push('\x00');
    new_mem
}
pub fn create_string(name: String, memory: Memory) -> Memory {
    let mut new_mem = memory;
    (new_mem.3).0.push(name);
    (new_mem.3).1.push(String::new());
    new_mem
}