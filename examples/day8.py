data = []
for line in open("inputs/day8.txt"):
    #for line in open("test.txt"):
    line = line.strip().split()
    data.append((line[0], int(line[1])))


# part 1
acc = 0
visited = set()
address = 0
while True:
    if address in visited:
        break
    visited.add(address)
    instr, value = data[address]
    if instr == "jmp":
        address += value
        continue
    elif instr == "acc":
        acc += value
        address += 1
        continue
    else:
        address += 1
print("part 1", acc)

def run_program(code):
    acc = 0
    visited = set()
    address = 0
    while True:
        if address in visited:
            #print("terminated on ", address)
            return (False, acc)
        if address == len(code):
            return (True, acc)
        visited.add(address)
        instr, value = code[address]
        if instr == "jmp":
            address += value
            continue
        elif instr == "acc":
            acc += value
            address += 1
            continue
        else:
            address += 1

changeable = [ i for (i, (instr, _)) in enumerate(data) if instr != "acc"]
print(f"brute forcing {len(changeable)}")

for index, addr in enumerate(changeable):
    if index % 20 == 0:
        print(f"{index}/{len(changeable)}")

    code = data.copy()
    instr, value = code[addr]
    if instr == "nop":
        code[addr] = ("jmp", value)
    elif instr == "jmp":
        code[addr] = ("nop", value)
    else:
        raise ValueError("bad index")


    safe_exit, acc = run_program(code)
    if safe_exit:
        print("safe exit ", acc)