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
        .target("riscv32imac-unknown-none-elf")
        .shared_flag(true)
        .pic(false)
        .compile("mem");
}