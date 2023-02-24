use std::{env, fs};

#[derive(Clone, Copy)]
enum Opcode {
    ADD,
    ADDI,
    SUB,
    SUBI,
    MUL,
    DIV,
    MOD,
    COPY,
    COPYI,
    LOAD,
    LOADI,
    STORE,
    CMP,
    CMPI,
    BRANCHE,
    BRANCHG,
    JUMP,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String;
    if args.len() == 1 {
        path = String::from("programs/blur.txt");
    } else {
        let program_name = &args[1];
        path = format!("programs/{}", program_name);
    }
    let reader = fs::read_to_string(path).expect("Failed to read file.");
    let program: Vec<(Opcode, i32, i32)> = reader.lines().map(parse_line).collect();

    let mut registers = [0; 16];
    let mut memory = [0; 128];

    let mut program_counter: usize = 0;

    let mut cycles = 0;
    let mut instructions_executed = 0;

    memory[0] = 3;
    memory[1] = 3;
    memory[2] = 6;
    memory[3] = 9;
    memory[4] = 5;
    memory[5] = 2;
    memory[6] = 1;
    memory[7] = 6;
    memory[8] = 7;
    memory[9] = 8;
    memory[10] = 2;

    while program_counter < program.len() {
        let instruction = fetch(&program, &mut program_counter);
        decode();
        execute(
            &instruction,
            &mut registers,
            &mut memory,
            &mut program_counter,
        );

        cycles += 1;
        instructions_executed += 1;
    }

    println!("Done!\n");
    println!("Registers: {:?}\n", registers);
    println!("Memory: {:?}\n", memory);
    println!("Number of instructions executed: {instructions_executed}");
    println!("Number of cycles taken: {cycles}");
    let ipc: f32 = instructions_executed as f32 / cycles as f32;
    println!("Instructions per cycle (IPC): {:.2}", ipc);
}

fn parse_line(line: &str) -> (Opcode, i32, i32) {
    let tokens: Vec<&str> = line.split(' ').collect();
    let opcode = str_to_opcode(tokens[0]);
    let arg1 = tokens[1]
        .parse()
        .expect(&format!("Couldn't parse first operand of: {line}."));
    let arg2 = tokens[2]
        .parse()
        .expect(&format!("Couldn't parse second operand of: {line}."));
    return (opcode, arg1, arg2);
}

fn str_to_opcode(input: &str) -> Opcode {
    match input {
        "ADD" => Opcode::ADD,
        "ADDI" => Opcode::ADDI,
        "SUB" => Opcode::SUB,
        "SUBI" => Opcode::SUBI,
        "MUL" => Opcode::MUL,
        "DIV" => Opcode::DIV,
        "MOD" => Opcode::MOD,
        "COPY" => Opcode::COPY,
        "COPYI" => Opcode::COPYI,
        "LOAD" => Opcode::LOAD,
        "LOADI" => Opcode::LOADI,
        "STORE" => Opcode::STORE,
        "CMP" => Opcode::CMP,
        "CMPI" => Opcode::CMPI,
        "BRANCHE" => Opcode::BRANCHE,
        "BRANCHG" => Opcode::BRANCHG,
        "JUMP" => Opcode::JUMP,
        _ => panic!("Invalid opcode {}", input),
    }
}

fn fetch(program: &[(Opcode, i32, i32)], program_counter: &mut usize) -> (Opcode, i32, i32) {
    let instruction = program[*program_counter];
    *program_counter += 1;
    return instruction;
}

fn decode() {}

fn execute(
    instruction: &(Opcode, i32, i32),
    registers: &mut [i32],
    memory: &mut [i32],
    program_counter: &mut usize,
) {
    match &instruction.0 {
        Opcode::ADD => registers[instruction.1 as usize] += registers[instruction.2 as usize],
        Opcode::ADDI => registers[instruction.1 as usize] += instruction.2,
        Opcode::SUB => registers[instruction.1 as usize] -= registers[instruction.2 as usize],
        Opcode::SUBI => registers[instruction.1 as usize] -= instruction.2,
        Opcode::MUL => registers[instruction.1 as usize] *= registers[instruction.2 as usize],
        Opcode::DIV => registers[instruction.1 as usize] /= registers[instruction.2 as usize],
        Opcode::MOD => registers[instruction.1 as usize] %= registers[instruction.2 as usize],
        Opcode::COPY => registers[instruction.1 as usize] = registers[instruction.2 as usize],
        Opcode::COPYI => registers[instruction.1 as usize] = instruction.2,
        Opcode::LOAD => {
            registers[instruction.1 as usize] = memory[registers[instruction.2 as usize] as usize]
        }
        Opcode::LOADI => registers[instruction.1 as usize] = memory[instruction.2 as usize],
        Opcode::STORE => {
            memory[registers[instruction.1 as usize] as usize] = registers[instruction.2 as usize]
        }
        Opcode::CMP => {
            if registers[instruction.1 as usize] == registers[instruction.2 as usize] {
                registers[instruction.1 as usize] = 0
            } else if registers[instruction.1 as usize] > registers[instruction.2 as usize] {
                registers[instruction.1 as usize] = -1
            } else {
                registers[instruction.1 as usize] = 1
            }
        }
        Opcode::CMPI => {
            if registers[instruction.1 as usize] == instruction.2 {
                registers[instruction.1 as usize] = 0
            } else if registers[instruction.1 as usize] > instruction.2 {
                registers[instruction.1 as usize] = -1
            } else {
                registers[instruction.1 as usize] = 1
            }
        }
        Opcode::BRANCHE => {
            if registers[instruction.1 as usize] == 0 {
                *program_counter += instruction.2 as usize - 1;
            }
        }
        Opcode::BRANCHG => {
            if registers[instruction.1 as usize] == 1 {
                *program_counter += instruction.2 as usize - 1;
            }
        }
        Opcode::JUMP => *program_counter += instruction.1 as usize - 1,
    }
}
