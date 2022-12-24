#!/usr/bin/env python 
from sys import argv

b = []
with open(argv[1], "rb") as f:
    while (byte := f.read(1)):
        b.append(hex(int.from_bytes(byte, "little")))

print(b[0x100])
print(b[0x101])
print(b[0x213])
print(b[0x214])
