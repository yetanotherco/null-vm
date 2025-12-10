PROGRAMS_DIR=./programs
ARTIFACTS_DIR=./program_artifacts


ASM_PROGRAMS = $(wildcard $(PROGRAMS_DIR)/*.s)
ARTIFACTS_ASM = $(patsubst $(PROGRAMS_DIR)/%.s, $(ARTIFACTS_DIR)/%.elf, $(ASM_PROGRAMS))

compile-programs-asm: clean $(ARTIFACTS_ASM)

# Compile assembly .s -> .o
$(ARTIFACTS_DIR)/%.o: $(PROGRAMS_DIR)/%.s
	clang --target=riscv32 -c $< -o $@

# Link assembly .o -> .elf
$(ARTIFACTS_DIR)/%.elf: $(ARTIFACTS_DIR)/%.o
	riscv64-unknown-elf-ld -m elf32lriscv $< -o $@ -e main

clean:
	-rm -rf $(ARTIFACTS_DIR)
	mkdir -p $(ARTIFACTS_DIR)

test: compile-programs-asm
	cargo test
