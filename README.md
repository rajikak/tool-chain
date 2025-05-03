A simple toolchain(file, strings, assembler, linker, loader, disassembler) written in Rust.

## ELF File Formats
* Linux ([Source](https://ics.uci.edu/~aburtsev/238P/hw/hw3-elf/hw3-elf.html#4))
![Linux](assets/linux.png)

* MacOS ([Source](https://github.com/aidansteele/osx-abi-macho-file-format-reference/blob/master/Mach-O_File_Format.pdf))
![MacOS](assets/mac.png)

## Toolchain
* Object (binaries, shared libs, core dumps) files can be analyzed using `objdump` and `otool` in MacOS and `readelf` in Linux.
```
//hello.c 
#include <stdio.h>

int main() {
	printf("hello world!\n");
} 
```

* Use `clang` to generate the object file. 
```
$ clang -o hello.o hello.c 
```
Using Linux:
```
$ gcc -o hello.o hello.c 
```

  
## Reading ELF Details

* `readelf` with `-a` produces all details, while `-h` gives ELF header:
```
$ readelf -h hello.o
[ec2-user@ip-172-31-17-175 linker]$ readelf -h hello.o
ELF Header:
Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
Class:                             ELF64
Data:                              2's complement, little endian
Version:                           1 (current)
OS/ABI:                            UNIX - System V
ABI Version:                       0
Type:                              EXEC (Executable file)
Machine:                           Advanced Micro Devices X86-64
Version:                           0x1
Entry point address:               0x400400
Start of program headers:          64 (bytes into file)
Start of section headers:          6320 (bytes into file)
Flags:                             0x0
Size of this header:               64 (bytes)
Size of program headers:           56 (bytes)
Number of program headers:         9
Size of section headers:           64 (bytes)
Number of section headers:         29
Section header string table index: 28
```


Using `objdump` to view the object file content (Linux: `readelf -a hello.o`).
```
$ objdump -p hello.o 
$ otool -l hello.o 
```


# Reference Materials
* [A ToC of the 20 part linker essay](https://lwn.net/Articles/276782/)
* [I wrote a linker everyone can understand!](https://briancallahan.net/blog/20210609.html)
* [Linkers and Loaders - John R. Levine](https://www.amazon.com/Linkers-Kaufmann-Software-Engineering-Programming/dp/1558604960)
* [Toolchains](https://www.toolchains.net/)
* [OS X ABI Mach-O File Format Reference](https://github.com/aidansteele/osx-abi-macho-file-format-reference)
* [The ELF format](https://ics.uci.edu/~aburtsev/238P/hw/hw3-elf/hw3-elf.html)
* [ELF at OSDev](https://wiki.osdev.org/ELF)
