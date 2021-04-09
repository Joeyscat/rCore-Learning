
### command not found: qemu-riscv64
```shell
➜  os git:(main) ✗ qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os; echo $?
zsh: command not found: qemu-riscv64
127
➜  os git:(main) ✗ sudo apt-get install qemu-user        
Reading package lists... Done
...
➜  os git:(main) ✗ qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os; echo $?
Hello World!
9
```