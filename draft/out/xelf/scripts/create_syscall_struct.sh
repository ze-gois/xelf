#!/bin/bash

# Array of directories and their files
declare -A syscalls=(
    ["openat"]="mod.rs flags.rs"
    ["read"]="mod.rs"
    ["write"]="mod.rs"
    ["close"]="mod.rs"
    ["lseek"]="mod.rs flags.rs"
    ["exit"]="mod.rs"
    ["mmap"]="mod.rs flags.rs"
    ["munmap"]="mod.rs"
    ["mprotect"]="mod.rs flags.rs"
)

# Create directories and files
for syscall in "${!syscalls[@]}"; do
    mkdir -p "$syscall"

    # Create files for each directory
    for file in ${syscalls[$syscall]}; do
        touch "$syscall/$file"
    done
done
