```bash
➜  rcore-learning git:(第二章/实现批处理操作系统+应用程序) cd user 
➜  user git:(第二章/实现批处理操作系统+应用程序) make build
    Finished release [optimized] target(s) in 0.00s
src/bin/00hello_world.rs src/bin/01store_fault.rs src/bin/02power.rs
target/riscv64gc-unknown-none-elf/release/00hello_world target/riscv64gc-unknown-none-elf/release/01store_fault target/riscv64gc-unknown-none-elf/release/02power
target/riscv64gc-unknown-none-elf/release/00hello_world.bin target/riscv64gc-unknown-none-elf/release/01store_fault.bin target/riscv64gc-unknown-none-elf/release/02power.bin
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/00hello_world --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/00hello_world.bin;  rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/01store_fault --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/01store_fault.bin;  rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/02power --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/02power.bin;
➜  user git:(第二章/实现批处理操作系统+应用程序) cd ../os
➜  os git:(第二章/实现批处理操作系统+应用程序) make run
(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
riscv64gc-unknown-none-elf (installed)
cargo install cargo-binutils --vers ~0.2
    Updating `https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index` index
     Ignored package `cargo-binutils v0.2.0` is already installed, use --force to override
rustup component add rust-src
info: component 'rust-src' is up to date
rustup component add llvm-tools-preview
info: component 'llvm-tools-preview' for target 'x86_64-unknown-linux-gnu' is up to date
make[1]: Entering directory '/home/jojo/code/rust/rcore-learning/user'
    Finished release [optimized] target(s) in 0.00s
src/bin/00hello_world.rs src/bin/01store_fault.rs src/bin/02power.rs
target/riscv64gc-unknown-none-elf/release/00hello_world target/riscv64gc-unknown-none-elf/release/01store_fault target/riscv64gc-unknown-none-elf/release/02power
target/riscv64gc-unknown-none-elf/release/00hello_world.bin target/riscv64gc-unknown-none-elf/release/01store_fault.bin target/riscv64gc-unknown-none-elf/release/02power.bin
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/00hello_world --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/00hello_world.bin;  rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/01store_fault --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/01store_fault.bin;  rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/02power --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/02power.bin;
make[1]: Leaving directory '/home/jojo/code/rust/rcore-learning/user'
Platform: qemu
   Compiling os v0.1.0 (/home/jojo/code/rust/rcore-learning/os)
    Finished release [optimized] target(s) in 0.28s
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
[kernel] Hello World!
[kernel] num_app = 3
[kernel] app_0 [0x8023b028, 0x8026f038)
[kernel] app_1 [0x8026f038, 0x802a30c8)
[kernel] app_2 [0x802a30c8, 0x802d72a8)
[kernel] Loading app_0
Hello World!
scause=0x2
[kernel] IlliegalInstruction in application, core dumped.
[kernel] Loading app_1
Into Test tore_fault, we will insert an invalid store operation...
Kernel should this application!
[kernel] PageFault in application, core dumped.
[kernel] Loading app_2
3^10000=5079
3^20000=8202
3^30000=8824
3^40000=5750
3^50000=3824
3^60000=8516
3^70000=2510
3^80000=9379
3^90000=2621
3^100000=2749
Test power OK!
[kernel] Application exited with code 0
[kernel] Panicked at src/batch.rs:77 All applications completed!
```