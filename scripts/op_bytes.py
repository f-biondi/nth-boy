import json

doc = json.load(open("Opcodes.json"))

b_op = []
for i,o in enumerate(doc["unprefixed"]):
    b_op.append(doc["unprefixed"][o]["bytes"])

print(b_op)

b_op = []
for i,o in enumerate(doc["cbprefixed"]):
    b_op.append(doc["cbprefixed"][o]["bytes"])


print(b_op)
