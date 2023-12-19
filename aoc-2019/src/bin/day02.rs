#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

enum Op {
    Add(usize, usize, usize), // source, source, dest
    Mul(usize, usize, usize), // source, source, dest
    Halt,
    Unknown(usize),
}

impl Op {
    const fn parse(instructions: &[usize]) -> Self {
        match instructions[0] {
            1 => Self::Add(instructions[1], instructions[2], instructions[3]),
            2 => Self::Mul(instructions[1], instructions[2], instructions[3]),
            99 => Self::Halt,
            n => Self::Unknown(n),
        }
    }
}

#[derive(Debug)]
struct IntCode {
    memory: Vec<usize>,
    instruction_pointer: usize,
}

impl IntCode {
    fn new(memory: Vec<usize>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
        }
    }
    fn step(&mut self) -> bool {
        match Op::parse(&self.memory[self.instruction_pointer..]) {
            Op::Add(source1, source2, dest) => {
                self.memory[dest] = self.memory[source1] + self.memory[source2];
            }
            Op::Mul(source1, source2, dest) => {
                self.memory[dest] = self.memory[source1] * self.memory[source2];
            }
            Op::Halt => return false,
            Op::Unknown(n) => panic!("Unknown instruction {n}"),
        }
        self.instruction_pointer += 4;
        true
    }
}

fn main() {
    let memory = aoc_helpers::include_data!(day02)
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let (noun, verb) = (0..=99)
        .flat_map(|noun: usize| (0..=99).map(move |verb: usize| (noun, verb)))
        .find(|&(noun, verb)| {
            let mut intcode = IntCode::new(memory.clone());
            intcode.memory[1] = noun;
            intcode.memory[2] = verb;
            while intcode.step() {}
            intcode.memory[0] == 19_690_720
        })
        .unwrap();
    println!("Day 02 result: {}", 100 * noun + verb);
}
