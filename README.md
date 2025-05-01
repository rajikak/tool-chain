A simple toolchain(assembler, linker, loader) written in Rust.

# Tool Chain 
```
//hello.c 
#include <stdio.h>

int main() {
	printf("hello world!\n");
} 
```

Generate `clang` to generate the object file. 
```
$ clang -o hello.o hello.c 
```
 
`objdump` can be used to view the object file content.
```
$ objdump --section-headers hello.o
hello.o:	file format mach-o arm64

Sections:
Idx Name          Size     VMA              Type
  0 __text        00000020 0000000100003f7c TEXT
  1 __stubs       0000000c 0000000100003f9c TEXT
  2 __cstring     0000000e 0000000100003fa8 DATA
  3 __unwind_info 00000048 0000000100003fb8 DATA
  4 __got         00000008 0000000100004000 DATA  
```

# Reference Materials
* [A ToC of the 20 part linker essay](https://lwn.net/Articles/276782/)
* [I wrote a linker everyone can understand!](https://briancallahan.net/blog/20210609.html)
* [Linkers and Loaders - John R. Levine](https://www.amazon.com/Linkers-Kaufmann-Software-Engineering-Programming/dp/1558604960)
* [Toolchains](https://www.toolchains.net/)
* [OS X ABI Mach-O File Format Reference](https://github.com/aidansteele/osx-abi-macho-file-format-reference)
