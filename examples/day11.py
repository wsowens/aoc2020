from pprint import pprint
DATA = [ list(line.strip()) for line in open("inputs/day11.txt") ]
TEST = [ list(line.strip()) for line in open("test") ]
def neighbors(i, j, data):
    for u in range(i-1, i+2):
        for v in range(j-1, j+2):
            if i == u and v == j:
                continue
            if u < 0 or v < 0:
                continue
            try:
                yield data[u][v]
            except IndexError:
                continue

from copy import deepcopy

def iterate(data):
    updated = deepcopy(data)
    for i, row in enumerate(data):
        for j, seat in enumerate(row):
            n = list(neighbors(i, j, data))
            #print(i, j, "occupied: ", len(list(filter(lambda x: x == "#", n))))
            if seat == "L" and all(map(lambda x: x != "#", n)):
                updated[i][j] = "#"
            elif seat == "#" and len(list(filter(lambda x: x == "#", n))) >= 4:
                updated[i][j] = "L"
    return updated

def view(d):
    pprint(["".join(row) for row in d])

#pprint(DATA)
#pprint(TEST)

#DATA = TEST
'''
DATA, PREV = (iterate(DATA), DATA)
count = 0
while PREV != DATA:
    PREV = DATA
    DATA = iterate(DATA)
    count += 1

view(DATA)

count = 0
for row in DATA:
    for v in row:
        if v == "#":
            count += 1
print(count)
'''

def look_out(i, j, i_step, j_step, data):
    while True:
        i += i_step
        j += j_step
        if i < 0 or j < 0:
            return False
        try:
            if data[i][j] == "#":
                return True
            elif data[i][j] == "L":
                return False
        except IndexError:
            return False

def look_around(i, j, data):
    count = 0
    for i_step in range(-1, 2):
        for j_step in range(-1, 2):
            if i_step == 0 and j_step == 0:
                continue
            if look_out(i, j, i_step, j_step, data):
                count += 1
    return count

def iterate(data):
    updated = deepcopy(data)
    for i, row in enumerate(data):
        for j, seat in enumerate(row):
            n = look_around(i, j, data)
            #print(i, j, "occupied: ", len(list(filter(lambda x: x == "#", n))))
            if seat == "L" and n == 0:
                updated[i][j] = "#"
            elif seat == "#" and n >= 5:
                updated[i][j] = "L"
    return updated

#DATA = TEST
DATA, PREV = (iterate(DATA), DATA)
count = 0
while PREV != DATA:
    PREV = DATA
    DATA = iterate(DATA)
    count += 1

view(DATA)

count = 0
for row in DATA:
    for v in row:
        if v == "#":
            count += 1
print(count)