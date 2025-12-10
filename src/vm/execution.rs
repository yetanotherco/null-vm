use std::{collections::BTreeMap, fmt::Debug};

use crate::vm::instructions::{ArithOp, Comparison, Instruction, LoadStoreWidth};

pub fn run_program(instruction_map: BTreeMap<u32, u32>, entrypoint: u32) {
    let mut memory = Memory::default();
    load_program(instruction_map, &mut memory);
    run_from_entrypoint(&mut memory, entrypoint);
}

fn load_program(instruction_map: BTreeMap<u32, u32>, memory: &mut Memory) {
    for (addr, instruction) in instruction_map {
        memory.0.insert(addr, instruction);
    }
}

fn run_from_entrypoint(memory: &mut Memory, entrypoint: u32) {
    let mut pc = entrypoint;
    dbg!(&pc, &memory);
    let mut registers = Registers::default();
    while pc != registers.0[1] {
        let next_instruction = memory.0[&pc];
        let instruction = Instruction::parse(next_instruction);
        run_instruction(&instruction, &mut registers, &mut pc, memory);
    }
    dbg!(&registers);
    let return_values = (registers.0[10], registers.0[11]);
    println!("Return Values: {return_values:?}");
}

// Toy Memory, TODO: Make expandable memory
#[derive(Default, Debug)]
struct Memory(BTreeMap<u32, u32>);

#[derive(Default)]
struct Registers([u32; 32]);
// Registers:
// 0x zero
// a0-ax function arguments: 0x10 -etc
// 0x1 return address (ra)
//
impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, elem) in self.0.iter().enumerate() {
            match i {
                1 => format!("ra: {elem}").fmt(f)?,
                2 => format!("sp: {elem}").fmt(f)?,
                3 => format!("gp: {elem}").fmt(f)?,
                i @ 10..17 => format!("a{} : {}", i - 10, elem).fmt(f)?,
                _ => {}
            }
        }
        Ok(())
    }
}

fn run_instruction(
    inst: &Instruction,
    registers: &mut Registers,
    pc: &mut u32,
    memory: &mut Memory,
) {
    *pc += 4;
    dbg!(inst);
    match inst {
        Instruction::ArithImm { dst, src, imm, op } => {
            let (a, b) = (registers.0[*src as usize], imm);
            let res = match op {
                ArithOp::Add => a + b,
                _ => unimplemented!(),
            };
            registers.0[*dst as usize] = res;
        }
        Instruction::JumpAndLinkRegister { dst, base, offset } => {
            registers.0[*dst as usize] = *pc;
            *pc = registers.0[*base as usize] + offset;
        }
        Instruction::JumpAndLink { dst, offset } => {
            registers.0[*dst as usize] = *pc;
            *pc += offset;
        }
        Instruction::Store {
            src,
            offset,
            base,
            width,
        } => {
            let value = registers.0[*src as usize];
            let value = match width {
                LoadStoreWidth::Byte => todo!(),
                LoadStoreWidth::Half => todo!(),
                LoadStoreWidth::Word => value,
            };
            memory.0.insert(*base + *offset, value);
        }
        Instruction::Load {
            dst,
            offset,
            base,
            width,
        } => {
            let value = memory.0[&(*base + *offset)];
            let value = match width {
                LoadStoreWidth::Byte => todo!(),
                LoadStoreWidth::Half => todo!(),
                LoadStoreWidth::Word => value,
            };
            registers.0[*dst as usize] = value;
        }
        Instruction::Branch {
            src1,
            src2,
            cond,
            offset,
        } => {
            let (a, b) = (registers.0[*src1 as usize], registers.0[*src2 as usize]);
            let cmp_result = match cond {
                Comparison::Equal => a == b,
                Comparison::NotEqual => a != b,
                Comparison::LessThan => a < b,
                Comparison::GreaterOrEqual => a >= b,
            };
            if cmp_result {
                *pc += offset
            }
        }
        _ => unimplemented!(),
    }
}
