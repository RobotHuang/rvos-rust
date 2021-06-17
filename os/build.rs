fn main() {
    cc::Build::new()
        .no_default_flags(true)
        .flag("-nostdlib")
        .flag("-fno-builtin")
        .flag("-march=rv32ima")
        .flag("-mabi=ilp32")
        .flag("-g")
        .flag("-Wall")
        .file("src/mem.S")
        .file("src/printf.c")
        .file("src/uart.c")
        .file("src/entry.S")
        .target("riscv32imac-unknown-none-elf")
        .pic(false)
        .compile("mem");
}
