A simple toolchain(file, strings, strip, readelf, assembler, linker, loader, disassembler) written in Rust.

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
* Using `gcc`.
```
$ gcc -o hello.o hello.c 
```

  
## Reading ELF Details

* All details:
```
$ readelf -a hello.o
```

* ELF header:
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
On MacOS( or `objdump -p hello.o`): 
```
￫ otool -h hello.o 
hello.o:
Mach header
      magic  cputype cpusubtype  caps    filetype ncmds sizeofcmds      flags
 0xfeedfacf 16777228          0  0x00           2    17       1056 0x00200085
￫ 
￫ 
￫ hexdump -n 20  hello.o 
0000000 facf feed 000c 0100 0000 0000 0002 0000
0000010 0011 0000                              
0000014 
```


* All segments:
```
[ec2-user@ip-172-31-17-175 linker]$ readelf --segments hello.o 

Elf file type is EXEC (Executable file)
Entry point 0x400400
There are 9 program headers, starting at offset 64

Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  PHDR           0x0000000000000040 0x0000000000400040 0x0000000000400040
                 0x00000000000001f8 0x00000000000001f8  R E    0x8
  INTERP         0x0000000000000238 0x0000000000400238 0x0000000000400238
                 0x000000000000001c 0x000000000000001c  R      0x1
      [Requesting program interpreter: /lib64/ld-linux-x86-64.so.2]
  LOAD           0x0000000000000000 0x0000000000400000 0x0000000000400000
                 0x0000000000000698 0x0000000000000698  R E    0x200000
  LOAD           0x0000000000000e10 0x0000000000600e10 0x0000000000600e10
                 0x0000000000000214 0x0000000000000218  RW     0x200000
  DYNAMIC        0x0000000000000e20 0x0000000000600e20 0x0000000000600e20
                 0x00000000000001d0 0x00000000000001d0  RW     0x8
  NOTE           0x0000000000000254 0x0000000000400254 0x0000000000400254
                 0x0000000000000044 0x0000000000000044  R      0x4
  GNU_EH_FRAME   0x0000000000000570 0x0000000000400570 0x0000000000400570
                 0x0000000000000034 0x0000000000000034  R      0x4
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10
  GNU_RELRO      0x0000000000000e10 0x0000000000600e10 0x0000000000600e10
                 0x00000000000001f0 0x00000000000001f0  R      0x1

 Section to Segment mapping:
  Segment Sections...
   00     
   01     .interp 
   02     .interp .note.ABI-tag .note.gnu.build-id .gnu.hash .dynsym .dynstr .gnu.version .gnu.version_r .rela.dyn .rela.plt .init .plt .text .fini .rodata .eh_frame_hdr .eh_frame 
   03     .init_array .fini_array .dynamic .got .got.plt .data .bss 
   04     .dynamic 
   05     .note.ABI-tag .note.gnu.build-id 
   06     .eh_frame_hdr 
   07     
   08     .init_array .fini_array .dynamic .got 
```

Using `objdump` to view the object file content.
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
* [Executable and Linkable Format (ELF)](https://www.cs.cmu.edu/afs/cs/academic/class/15213-f00/docs/elf.pdf)
* [ELF Spec](https://refspecs.linuxfoundation.org/elf/elf.pdf)
* [The ELF format](https://ics.uci.edu/~aburtsev/238P/hw/hw3-elf/hw3-elf.html)
* [ELF at OSDev](https://wiki.osdev.org/ELF)
* [Introduction to the Linux ELF file](https://www.youtube.com/watch?v=1VnnbpHDBBA&t=17s)
* [ELF in Depth](https://www.youtube.com/watch?v=nC1U1LJQL8o)
* [elf.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h)
* [ELF Format Cheatsheet](https://gist.github.com/x0nu11byt3/bcb35c3de461e5fb66173071a2379779)
