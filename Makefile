ASM_PROGRAMS_DIR=./programs/asm
ASM_ARTIFACTS_DIR=./program_artifacts/asm


ASM_PROGRAMS = $(wildcard $(ASM_PROGRAMS_DIR)/*.s)
ARTIFACTS_ASM = $(patsubst $(ASM_PROGRAMS_DIR)/%.s, $(ASM_ARTIFACTS_DIR)/%.elf, $(ASM_PROGRAMS))

compile-programs-asm: clean $(ARTIFACTS_ASM)

# Compile assembly .s -> .o
$(ASM_ARTIFACTS_DIR)/%.o: $(ASM_PROGRAMS_DIR)/%.s
	clang --target=riscv32 -c $< -o $@

# Link assembly .o -> .elf
$(ASM_ARTIFACTS_DIR)/%.elf: $(ASM_ARTIFACTS_DIR)/%.o
	riscv64-unknown-elf-ld -m elf32lriscv $< -o $@ -e main

clean:
	-rm -rf $(ASM_ARTIFACTS_DIR)
	mkdir -p $(ASM_ARTIFACTS_DIR)

test: compile-programs-asm
	cargo test
