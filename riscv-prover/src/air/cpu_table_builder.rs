use crate::air::cpu_air::build_cpu_trace;
use lambdaworks_math::field::{
    element::FieldElement,
    fields::fft_friendly::{
        babybear_u32::Babybear31PrimeField, quartic_babybear_u32::Degree4BabyBearU32ExtensionField,
    },
};
use stark_platinum_prover::trace::TraceTable;

type FE = FieldElement<Babybear31PrimeField>;

type CPUTraceTable = TraceTable<Babybear31PrimeField, Degree4BabyBearU32ExtensionField>;

/// Basic instructions supported initially: ADD, MUL, OR, STORE
#[derive(Debug, Clone, Copy)]
pub enum SimpleOpcode {
    ADD,   // Column 15
    MUL,   // Column 28
    OR,    // Column 19
    STORE, // Column 27
}

/// Simple representation of a CPU instruction

// Improve comments
#[derive(Debug, Clone)]
pub struct CPUInstruction {
    pub pc: u32,
    pub opcode: SimpleOpcode,
    pub rs1: Option<u8>,
    pub rs2: Option<u8>,
    pub rd: Option<u8>,
    pub imm: Option<u32>,
    pub rv1_value: u32,
    pub rv2_value: u32,
    pub result_value: u32,

    // If None, the rvd value is the result value
    pub rvd_value: Option<u32>,
    // If None, the next pc is the pc + 4
    pub next_pc: Option<u32>,
}

pub struct CPUTableBuilder {
    instructions: Vec<CPUInstruction>,
}

impl CPUTableBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: CPUInstruction) -> &mut Self {
        self.instructions.push(inst);
        self
    }

    pub fn build(&self) -> Vec<Vec<FE>> {
        let num_rows = self.instructions.len();

        let mut columns: Vec<Vec<FE>> = (0..54).map(|_| vec![FE::zero(); num_rows]).collect();

        for (row_idx, inst) in self.instructions.iter().enumerate() {
            self.fill_row(&mut columns, row_idx, inst);
        }

        columns
    }

    pub fn build_trace(&self) -> CPUTraceTable {
        let columns = self.build();
        build_cpu_trace(columns)
    }

    fn fill_row(&self, columns: &mut [Vec<FE>], row: usize, inst: &CPUInstruction) {
        // Timestamp (columns 0-1) - Word2L
        // Column index: 0 is zero for this small examples
        columns[0][row] = FE::zero();
        // Column index: 1
        let timestamp = 4 * (row + 1) as u32;
        columns[1][row] = FE::from(&timestamp);

        // Program Counter (columns 2-3) - Word2L
        // Column index: 2 is zero for small PC values
        columns[2][row] = FE::zero();
        // Column index: 3
        columns[3][row] = FE::from(&inst.pc);

        // Register indices (columns 4-6)
        // Column index: 4 - Index of source register 1
        columns[4][row] = FE::from(&(inst.rs1.unwrap_or(0) as u32));
        // Column index: 5 - Index of source register 2
        columns[5][row] = FE::from(&(inst.rs2.unwrap_or(0) as u32));
        // Column index: 6 - Index of destination register
        columns[6][row] = FE::from(&(inst.rd.unwrap_or(0) as u32));

        // Decode flags (columns 7-14)

        // Column index: 7 - Should the result be written to register
        let write_register = inst.rd.is_some() && inst.rd.unwrap_or(0) != 0;
        columns[7][row] = if write_register {
            FE::one()
        } else {
            FE::zero()
        };

        // Column index: 8 - 9 - Does memory access touch at least 2 bytes and 4 bytes
        // Not used for ADD/MUL/OR/STORE so these are set to zero (only for this example )
        columns[8][row] = FE::zero();
        columns[9][row] = FE::zero();

        // Immediate value (columns 10-11) - Word2L
        // Column index: 10 - 11 - Immediate value
        // For small immediate values, only column 11 has the value, others are zero
        columns[10][row] = FE::zero();
        // Column index: 11
        let imm_value = inst.imm.unwrap_or(0);
        columns[11][row] = FE::from(&imm_value);

        // Signed and muldiv flags (columns 12-14)
        // Not used for ADD/MUL/OR/STORE so these are set to zero (only for this example )
        // Column index: 12 signed flag
        columns[12][row] = FE::zero();
        // Column index: 13 - signed_2 flag
        columns[13][row] = FE::zero();
        // Column index: 14 - muldiv_selector flag
        columns[14][row] = FE::zero();

        // Instruction flags one-hot (columns 15-31)
        for col in 15..=31 {
            columns[col][row] = FE::zero();
        }
        // Set the corresponding flag based on opcode
        // Column index: 15 - ADD
        // Column index: 19 - OR
        // Column index: 27 - STORE
        // Column index: 28 - MUL
        match inst.opcode {
            SimpleOpcode::ADD => columns[15][row] = FE::one(),
            SimpleOpcode::MUL => columns[28][row] = FE::one(),
            SimpleOpcode::OR => columns[19][row] = FE::one(),
            SimpleOpcode::STORE => columns[27][row] = FE::one(),
        }

        // Next PC (columns 32-33) - Word2L
        // Column index: 32 is zero for small next_pc values
        columns[32][row] = FE::zero();
        // Column index: 33
        let next_pc = inst.next_pc.unwrap_or(inst.pc + 4);
        columns[33][row] = FE::from(&next_pc);

        // rv1 Word4L (columns 34-37)
        // Value read from source register 1
        // For small values: only column 37 has the value, others are zero
        columns[34][row] = FE::zero();
        columns[35][row] = FE::zero();
        columns[36][row] = FE::zero();
        columns[37][row] = FE::from(&inst.rv1_value);

        // rv2 Word4L (columns 38-41)
        // Value read from source register 2
        columns[38][row] = FE::zero();
        columns[39][row] = FE::zero();
        columns[40][row] = FE::zero();
        columns[41][row] = FE::from(&inst.rv2_value);

        // TODO: Instead of setting these values to zero, we can use some kind of
        // helper function to set the value to the correct column.
        // The same applies for other instructions.

        // rvd Word2L (columns 42-43)
        // Value written to destination register
        // Column index: 42 is zero for small rvd values
        columns[42][row] = FE::zero();
        // Column index: 43
        let rvd_value = inst.rvd_value.unwrap_or(inst.result_value);
        columns[43][row] = FE::from(&rvd_value);

        // arg2 Word4L (columns 44-47)
        // Second argument of ALU operation
        // For STORE: use imm, for others: use rv2
        let arg2 = match inst.opcode {
            SimpleOpcode::STORE => imm_value,
            _ => inst.rv2_value,
        };
        columns[44][row] = FE::zero();
        columns[45][row] = FE::zero();
        columns[46][row] = FE::zero();
        columns[47][row] = FE::from(&arg2);

        // res Word4L (columns 48-51)
        // ALU result
        columns[48][row] = FE::zero();
        columns[49][row] = FE::zero();
        columns[50][row] = FE::zero();
        columns[51][row] = FE::from(&inst.result_value);

        // is_equal flag (column 52)
        // Whether rv1 and arg2 are equal
        // For small values: just compare column 37 (rv1) and column 47 (arg2)
        columns[52][row] = if inst.rv1_value == arg2 {
            FE::one()
        } else {
            FE::zero()
        };

        // branch_cond flag (column 53)
        // Whether a branch is taken
        columns[53][row] = FE::zero(); // Always 0 for now
    }
}

impl Default for CPUTableBuilder {
    fn default() -> Self {
        Self::new()
    }
}
