use crate::air::{
    cpu_air::CPUTableAIR,
    cpu_table_builder::{CPUInstruction, CPUTableBuilder, SimpleOpcode},
};
use lambdaworks_crypto::fiat_shamir::default_transcript::DefaultTranscript;
use lambdaworks_math::field::fields::fft_friendly::quartic_babybear_u32::Degree4BabyBearU32ExtensionField;
use stark_platinum_prover::{
    proof::options::ProofOptions,
    prover::{IsStarkProver, Prover},
    verifier::{IsStarkVerifier, Verifier},
};

#[test]
fn test_prove_cpu_table_with_builder() {
    let mut builder = CPUTableBuilder::new();

    // Row 0: ADD instruction
    // ADD r3, r1, r2  (r3 = r1 + r2)
    // r1=10, r2=10, result=20
    builder.add_instruction(CPUInstruction {
        pc: 0,
        opcode: SimpleOpcode::ADD,
        rs1: Some(1),
        rs2: Some(5),
        rd: Some(9),
        imm: None,
        rv1_value: 10,
        rv2_value: 10,
        result_value: 20, // 10 + 10 = 20
        rvd_value: Some(15),
        next_pc: Some(4),
    });

    // Row 1: MUL instruction
    // MUL r5, r3, r4  (r5 = r3 * r4)
    // r3=20, r4=20, result=400
    builder.add_instruction(CPUInstruction {
        pc: 1,
        opcode: SimpleOpcode::MUL,
        rs1: Some(2),
        rs2: Some(6),
        rd: Some(10),
        imm: None,
        rv1_value: 20,
        rv2_value: 20,
        result_value: 400,
        rvd_value: Some(400),
        next_pc: Some(5),
    });

    // Row 2: OR instruction
    // OR r7, r5, r6  (r7 = r5 | r6)
    // r5=30, r6=30, result=30
    builder.add_instruction(CPUInstruction {
        pc: 2,
        opcode: SimpleOpcode::OR,
        rs1: Some(3),
        rs2: Some(7),
        rd: Some(11),
        imm: None,
        rv1_value: 30,
        rv2_value: 30,
        result_value: 30,
        rvd_value: Some(30),
        next_pc: Some(6),
    });

    // Row 3: STORE instruction
    // STORE r1, r2, imm  (store value from r2 at address r1 + imm)
    // r1=40, r2=40, imm=0, result=40 (rv1 + imm = 40 + 0 = 40)
    builder.add_instruction(CPUInstruction {
        pc: 3,
        opcode: SimpleOpcode::STORE,
        rs1: Some(4),
        rs2: Some(8),
        rd: None,
        imm: Some(0),
        rv1_value: 40,
        rv2_value: 40,
        result_value: 40,
        rvd_value: None,
        next_pc: Some(7),
    });

    let columns = builder.build();

    assert_eq!(columns.len(), 54);

    let num_rows = columns[0].len();
    assert_eq!(num_rows, 4);
    for col in &columns {
        assert_eq!(col.len(), num_rows);
    }

    let mut trace = builder.build_trace();
    let proof_options = ProofOptions::default_test_options();

    let proof = Prover::<CPUTableAIR>::prove(
        &mut trace,
        &(),
        &proof_options,
        DefaultTranscript::<Degree4BabyBearU32ExtensionField>::new(&[]),
    )
    .unwrap();

    assert!(Verifier::<CPUTableAIR>::verify(
        &proof,
        &(),
        &proof_options,
        DefaultTranscript::<Degree4BabyBearU32ExtensionField>::new(&[]),
    ));
}
