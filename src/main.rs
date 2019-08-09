use std::io;

fn main() {
    println!("Input your instruction :");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line = line.replace(" ", "").replace("\n", "");
    let line: Vec<&str> = line.split(':').collect();
    if line.len() != 2 || line[0].len() == 0 || line[1].len() == 0 {
        println!("Error : incorrect syntax");
    } else {
        let instruction = line[0];
        println!("\ninstruction : {:?}", instruction);

        let operands: Vec<&str> = line[1].split(",").collect();
        print!("List of operands : ");
        for operand in operands {
            print!("{:?} ; ", operand);
        }
        println!();
    }
}
