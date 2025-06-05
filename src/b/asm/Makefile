# Compiler and assembler
NASM = nasm
RUSTC = rustc
RUSTFLAGS = --target-dir=target --edition=2021

# Directories
LIB_SRC = lib/src
ASM_DIR = lib/src/arch
TARGET_DIR = target

# Source files
MAIN_SRC = src/main.rs
LIB_MAIN = $(LIB_SRC)/lib.rs
ASM_SRCS = $(wildcard $(ASM_DIR)/*.asm)
ASM_OBJS = $(ASM_SRCS:.asm=.o)

# Output files
OUTPUT = $(TARGET_DIR)/elf_loader
LIB_OUTPUT = $(TARGET_DIR)/libelf_loader.rlib

# Default target
all: $(OUTPUT)

# Compile the main binary
$(OUTPUT): $(MAIN_SRC) $(LIB_OUTPUT)
	@mkdir -p $(TARGET_DIR)
	$(RUSTC) $(RUSTFLAGS) --extern elf_loader=$(LIB_OUTPUT) $(MAIN_SRC) -o $(OUTPUT)

# Compile the library
$(LIB_OUTPUT): $(LIB_MAIN) $(ASM_OBJS)
	@mkdir -p $(TARGET_DIR)
	$(RUSTC) $(RUSTFLAGS) --crate-type=rlib $(LIB_MAIN) -o $(LIB_OUTPUT)

# Compile assembly files
%.o: %.asm
	$(NASM) -f elf64 $< -o $@

# Clean built files
clean:
	rm -rf $(TARGET_DIR)
	rm -f $(ASM_OBJS)

# Phony targets
.PHONY: all clean
