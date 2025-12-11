use null_vm::{elf::Elf, vm::execution::run_program};

#[test]
fn test_basic_program() {
    println!("Testing basic_program.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/basic_program.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == 0);
}

#[test]
fn test_addi_one() {
    println!("Testing addi_one.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_one.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == 1);
}

#[test]
fn test_addi_minus_one() {
    println!("Testing addi_minus_one.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_minus_one.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == -1);
}

#[test]
fn test_addi_max() {
    println!("Testing addi_max.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_max.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == 2047);
}

#[test]
fn test_addi_min() {
    println!("Testing addi_min.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_min.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == -2048);
}

#[test]
fn test_addi_reg() {
    println!("Testing addi_reg.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_reg.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == 30);
}

#[test]
fn test_addi_reg_max() {
    println!("Testing addi_reg_max.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_reg_max.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == 2080);
}

#[test]
fn test_addi_reg_min() {
    println!("Testing addi_reg_min.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/addi_reg_min.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == -2070);
}

#[test]
fn test_add() {
    println!("Testing add.elf");
    let elf_data = std::fs::read("./program_artifacts/asm/add.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });
    let results = run_program(program.image, program.entry_point);

    assert!(results.0 == 30);
}
