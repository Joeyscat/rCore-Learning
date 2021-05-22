### 查看编译生成的可执行文件
```shell
# 文件格式
➜  os git:(main) ✗ file target/riscv64gc-unknown-none-elf/debug/os
target/riscv64gc-unknown-none-elf/debug/os: ELF 64-bit LSB executable, UCB RISC-V, version 1 (SYSV), statically linked, with debug_info, not stripped
# 文件头信息
➜  os git:(main) ✗ rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os

File: target/riscv64gc-unknown-none-elf/debug/os
Format: elf64-littleriscv
Arch: riscv64
AddressSize: 64bit
LoadName: <Not found>
ElfHeader {
  ...
  Type: Executable (0x2)
  Machine: EM_RISCV (0xF3)
  Version: 1
  Entry: 0x119A0
  ...
}
# 反汇编导出汇编程序
➜  os git:(main) ✗ rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os

...

```


### 
```shell
# 编译生成ELF格式的执行文件
cargo build --release
 Compiling os v0.1.0 (/media/chyyuu/ca8c7ba6-51b7-41fc-8430-e29e31e5328f/thecode/rust/os_kernel_lab/os)
  Finished release [optimized] target(s) in 0.15s

# 把ELF执行文件转成bianary文件
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/os --strip-all -O binary target/riscv64gc-unknown-none-elf/release/os.bin

# 加载运行
qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```