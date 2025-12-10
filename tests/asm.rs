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
