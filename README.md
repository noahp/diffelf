[![Travis Build Status](https://img.shields.io/travis/com/noahp/diffelf.svg)](https://travis-ci.com/noahp/diffelf)
[![crates.io](https://img.shields.io/crates/v/diffelf.svg)](https://crates.io/crates/diffelf)


# diffelf

Show differences between ELF format files.

Usage (will be...):

```bash
# diff two elf files
diffelf file1.elf file2.elf
...

# unified diff style
diffelf -u file1.elf file2.elf
```
