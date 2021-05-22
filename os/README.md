
### command not found: qemu-riscv64
```bash
➜ os :  make run
(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
riscv64gc-unknown-none-elf (installed)
cargo install cargo-binutils --vers ~0.2
    Updating `git://mirrors.ustc.edu.cn/crates.io-index` index
     Ignored package `cargo-binutils v0.2.0` is already installed, use --force to override
rustup component add rust-src
info: component 'rust-src' is up to date
rustup component add llvm-tools-preview
info: component 'llvm-tools-preview' for target 'x86_64-unknown-linux-gnu' is up to date
Platform: qemu
    Finished release [optimized] target(s) in 0.00s
[rustsbi] RustSBI version 0.2.0-alpha.1
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Platform: QEMU (Version 0.2.0)
[rustsbi] misa: RV64ACDFIMSU
[rustsbi] mideleg: 0x222
[rustsbi] medeleg: 0xb1ab
[rustsbi-dtb] Hart count: cluster0 with 1 cores
[rustsbi] Kernel entry: 0x80200000
Hello World!
.text [0x80200000, 0x80221000)
.rodata [0x80221000, 0x80235000)
.data [0x80235000, 0x80236000)
.boot_stack [0x80236000, 0x80246000)
.bss [0x80246000, 0x80246000)
```