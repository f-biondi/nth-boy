import json

doc = json.load(open("Opcodes.json"))

b_op = []
for i,o in enumerate(doc["unprefixed"]):
    b_op.append(int(int(doc["unprefixed"][o]["cycles"][0])/4))

print(b_op)

