#!/usr/bin/env python 
from sys import argv

b = 0
with open(argv[1], "rb") as f:
    while (byte := f.read(1)):
        b+=1
        print("0x"+byte.hex()+",", end="" if b%8 else "\n")

