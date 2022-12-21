import json
import re

doc = json.load(open("Opcodes.json"))

def get_op_v(s, b, rst):
    if s[-1] == "H" and rst:
        return ("f8", "0x"+s[:-1])
    if s in ["AF", "BC", "DE", "HL", "SP", "PC"]:
        return ("r16", "Register16::"+s)
    elif s in ["A", "F", "B", "C", "D", "E", "H", "L"]:
        return ("r8", "Register8::"+s) if not b or s not in ["C","H"] else ("f", "Flag::"+s)
    elif s in ["Z", "N", "H", "C", "NZ", "NN", "NH", "NC"]:
        return ("f", "Flag::"+s)
    elif s == "d8":
        return ("u8",)
    elif s == "d16":
        return ("u16",)
    elif s == "a8":
        return ("u8",)
    elif s == "a16":
        return ("u16",)
    elif s == "r8":
        return ("i8",)

def get_b(name):
    return name in ["call","jr","jp","ret"]

for o in doc["unprefixed"]:
    name = doc["unprefixed"][o]["mnemonic"].lower() if "ILLEGAL" not in doc["unprefixed"][o]["mnemonic"] else "nop"
    params = []
    fres = name
    post_op = ""
    for op in doc["unprefixed"][o]["operands"]:
        op_name_res = "_"
        if not op["immediate"]:
            op_name_res += "i"
            if name == "ld":
                if "increment" in op:
                    post_op = "PostOp::INC"
                elif "decrement" in op:
                    post_op = "PostOp::DEC"
                else:
                    post_op = "PostOp::NONE"
        res = get_op_v(op["name"], get_b(name), name=="rst")
        op_name_res += res[0]
        fres += op_name_res
        if len(res) > 1:
            params.append(res[1])
    print(f"{o} => self.{fres}(", end="")
    for i,p in enumerate(params):
        if i:
            print(" ,", end="") 
        print(p, end="")
    if post_op != "":
        print(f", {post_op}", end="")
    print("),")

