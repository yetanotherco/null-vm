use null_vm::{elf::Elf, vm::execution::run_program};

fn run_program_and_check_output(elf_path: &str, expected_output: i32) {
    println!("Testing {}", elf_path);
    let elf_data = std::fs::read(elf_path).unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == expected_output);
}

#[test]
fn test_basic_program() {
    run_program_and_check_output(
        "./program_artifacts/asm/basic_program.elf",
        0,
    );
}

#[test]
fn test_addi_one() {
    run_program_and_check_output("./program_artifacts/asm/addi_one.elf", 1);
}

#[test]
fn test_addi_minus_one() {
    run_program_and_check_output("./program_artifacts/asm/addi_minus_one.elf", -1);
}

#[test]
fn test_addi_max() {
    run_program_and_check_output("./program_artifacts/asm/addi_max.elf", 2047);
}

#[test]
fn test_addi_min() {
    run_program_and_check_output("./program_artifacts/asm/addi_min.elf", -2048);
}

#[test]
fn test_addi_reg() {
    run_program_and_check_output("./program_artifacts/asm/addi_reg.elf", 30);
}

#[test]
fn test_addi_reg_max() {
    run_program_and_check_output("./program_artifacts/asm/addi_reg_max.elf", 2080);
}

#[test]
fn test_addi_reg_min() {
    run_program_and_check_output("./program_artifacts/asm/addi_reg_min.elf", -2070);
}

#[test]
fn test_add() {
    run_program_and_check_output("./program_artifacts/asm/add.elf", 30);
}

#[test]
fn test_add_neg() {
    run_program_and_check_output("./program_artifacts/asm/add_neg.elf", 10);
}

#[test]
fn test_add_max() {
    run_program_and_check_output("./program_artifacts/asm/add_max.elf", i32::MAX);
}

#[test]
fn test_add_max_plus_one() {
    run_program_and_check_output("./program_artifacts/asm/add_max_plus_one.elf", i32::MIN);
}

#[test]
fn test_add_min() {
    run_program_and_check_output("./program_artifacts/asm/add_min.elf", i32::MIN);
}

#[test]
fn test_add_min_minus_one() {
    run_program_and_check_output("./program_artifacts/asm/add_min_minus_one.elf", i32::MAX);
}
