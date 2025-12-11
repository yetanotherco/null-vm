use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

use crate::vm::instructions::{ArithOp, Comparison, Instruction, LoadStoreWidth};

pub fn run_program(instruction_map: BTreeMap<u32, u32>, entrypoint: u32) -> (i32, i32) {
    let mut memory = Memory::default();
    load_program(instruction_map, &mut memory);
    run_from_entrypoint(&mut memory, entrypoint)
}

fn load_program(instruction_map: BTreeMap<u32, u32>, memory: &mut Memory) {
    for (addr, instruction) in instruction_map {
        memory.0.insert(addr, instruction as i32);
    }
}

fn run_from_entrypoint(memory: &mut Memory, entrypoint: u32) -> (i32, i32) {
    let mut pc = entrypoint;
    let mut registers = Registers::default();
    // TODO: find what the starting value should be
    registers.0[2] = 16;
    while pc as i32 != registers.0[1] {
        let next_instruction = memory.0[&pc] as u32;
        let instruction = Instruction::parse(next_instruction);
        run_instruction(&instruction, &mut registers, &mut pc, memory);
    }
    println!("Final Register Values:\n {}", &registers);
    let return_values = (registers.0[10], registers.0[11]);
    println!("Return Values: {return_values:?}");
    return_values
}

// Toy Memory, TODO: Make expandable memory
#[derive(Default, Debug)]
struct Memory(BTreeMap<u32, i32>);

#[derive(Default, Debug)]
struct Registers([i32; 32]);
// Registers:
// 0x zero
// a0-ax function arguments: 0x10 -etc
// 0x1 return address (ra)
//
impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Registers:")?;
        writeln!(f, "Zero(zero): {}", self.0[0])?;
        writeln!(f, "ReturnAddress(ra): {}", self.0[1])?;
        writeln!(f, "StackPointer(sp): {}", self.0[2])?;
        // Not used for now
        // writeln!(f, "GlobalPointer(gp): {}", self.0[2])?;
        // writeln!(f, "ThreadPointer(tp): {}", self.0[3])?;
        let function_arguments = self.0[10..17]
            .iter()
            .enumerate()
            .map(|(i, val)| match i {
                i @ 0..=1 => format!("a{i} (return value {i}) : {val} "),
                i => format!("a{i}: {val} "),
            })
            .collect::<Vec<_>>()
            .concat();
        writeln!(f, "FunctionArguments: {function_arguments}")?;
        // TODO: Add other registers as we use them
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
    match inst {
        Instruction::ArithImm { dst, src, imm, op } => {
            let (a, b) = (registers.0[*src as usize], *imm);
            let res = match op {
                ArithOp::Add => a + b,
                ArithOp::Sub => panic!("SubImm not supported"),
                ArithOp::Xor => a ^ b,
                ArithOp::Or => a | b,
                ArithOp::And => a & b,
                ArithOp::ShiftLeftLogical => a << b,
                ArithOp::ShiftRightLogical => a >> b,
                ArithOp::ShiftRightArith => a >> b,
                ArithOp::SetLessThan => (a < b) as i32,
                ArithOp::SetLessThanU => ((a as u32) < (b as u32)) as i32,
            };
            registers.0[*dst as usize] = res;
        }
        Instruction::JumpAndLinkRegister { dst, base, offset } => {
            if *dst != 0 {
                registers.0[*dst as usize] = *pc as i32;
            }
            *pc = (registers.0[*base as usize] + offset) as u32;
        }
        Instruction::JumpAndLink { dst, offset } => {
            if *dst != 0 {
                registers.0[*dst as usize] = *pc as i32;
            }
            *pc -= 4;
            *pc = (*pc as i32 + offset) as u32;
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
            memory
                .0
                .insert(registers.0[*base as usize] as u32 + *offset, value);
        }
        Instruction::Load {
            dst,
            offset,
            base,
            width,
        } => {
            let value = memory.0[&((registers.0[*base as usize] + *offset) as u32)];
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
        Instruction::LoadUpperImm { dst, imm } => registers.0[*dst as usize] = (*imm << 12) as i32,
        Instruction::AddUpperImmToPc { dst, imm } => {
            registers.0[*dst as usize] = (*pc + (*imm << 12)) as i32
        }
        _ => unimplemented!(),
    }
}
