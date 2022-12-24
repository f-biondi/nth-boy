import json
import re

doc = json.load(open("Opcodes.json"))

def get_op_v(s):
    if s in ["0", "1", "2", "3", "4", "5", "6", "7"]:
        return ("b", s)
    elif s in ["AF", "BC", "DE", "HL", "SP", "PC"]:
        return ("r16", "Register16::"+s)
    elif s in ["A", "F", "B", "C", "D", "E", "H", "L"]:
        return ("r8", "Register8::"+s)
    
for o in doc["cbprefixed"]:
    name = doc["cbprefixed"][o]["mnemonic"].lower()
    params = []
    fres = name
    for op in doc["cbprefixed"][o]["operands"]:
        op_name_res = "_"
        if not op["immediate"]:
            op_name_res += "i"
        res = get_op_v(op["name"])
        op_name_res += res[0]
        fres += op_name_res
        if len(res) > 1:
            params.append(res[1])
    print(f"{o} => self.{fres}(", end="")
    for i,p in enumerate(params):
        if i:
            print(", ", end="") 
        print(p, end="")
    print("),")

