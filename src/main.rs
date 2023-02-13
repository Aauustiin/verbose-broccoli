use std::fs;

fn main() {
    let mut registers = [0; 16];
    let mut memory = [0; 128];

    let reader = fs::read_to_string("programs/test.txt").expect("Failed to read file.");
    let program: Vec<(&str, i32, i32)> = reader.lines().map(parse_line).collect();
    let mut program_counter: usize = 0;

    while program_counter < program.len() {
        let instruction = fetch(&program, &mut program_counter);
        decode();
        execute(&instruction, &mut registers, &mut memory);
    }

    println!("Done!\nRegisters: {:?}", registers);
}

fn parse_line(line: &str) -> (&str, i32, i32) {
    let tokens: Vec<&str> = line.split(' ').collect();
    let opcode = tokens[0];
    let arg1 = tokens[1]
        .parse()
        .expect(&format!("Couldn't parse first operand of: {line}."));
    let arg2 = tokens[2]
        .parse()
        .expect(&format!("Couldn't parse second operand of: {line}."));
    return (opcode, arg1, arg2);
}

fn fetch<'a>(
    program: &'a [(&'a str, i32, i32)],
    program_counter: &'a mut usize,
) -> (&'a str, i32, i32) {
    let instruction = program[*program_counter];
    *program_counter += 1;
    return instruction;
}

fn decode() {
    println!("Decode!");
}

fn execute(instruction: &(&str, i32, i32), registers: &mut [i32], memory: &mut [i32]) {
    match instruction.0 {
        "ADD" => registers[instruction.1 as usize] += registers[instruction.2 as usize],
        "ADDI" => registers[instruction.1 as usize] += instruction.2,
        "MUL" => registers[instruction.1 as usize] *= registers[instruction.2 as usize],
        "MULI" => registers[instruction.1 as usize] *= instruction.2,
        "DIV" => registers[instruction.1 as usize] /= registers[instruction.2 as usize],
        "DIVI" => registers[instruction.1 as usize] /= instruction.2,
        "MOV" => registers[instruction.1 as usize] = registers[instruction.2 as usize],
        "MOVI" => registers[instruction.1 as usize] = instruction.2,
        "LOAD" => registers[instruction.1 as usize] = memory[registers[instruction.2 as usize] as usize],
        "LOADI" => registers[instruction.1 as usize] = memory[instruction.2 as usize],
        "STORE" => memory[instruction.1 as usize] = registers[instruction.2 as usize],
        _ => {}
    }
}
