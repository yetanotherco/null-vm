use null_vm::elf::Elf;

#[test]
fn test_basic_program() {
    println!("Testing basic_program.elf");
    let elf_data = std::fs::read("./program_artifacts/basic_program.elf").unwrap();
    let program = Elf::load(&elf_data).unwrap();
    println!("Program entry: 0x{:08x}", program.entry_point);
    program.image.iter().for_each(|(addr, word)| {
        println!("0x{:08x}: 0x{:08x}", addr, word);
    });

    // todo: execute and check result
}
