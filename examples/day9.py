data = []
for line in open("inputs/day9.txt"):
    data.append(int(line.strip()))

for index, value in enumerate(data):
    if index < 25:
        continue
    prev_25 = data[index-25:index]
    sums = []
    for i1, v1 in enumerate(prev_25):
        for i2, v2 in enumerate(prev_25):
            if i1 != i2:
                sums.append(v1 + v2)
    if not value in sums:
        print("invalid:", index, value)
        invalid = value

for offset in range(len(data)):
    cumsum = 0
    start = offset
    stop = offset
    for index, value in enumerate(data[offset:]):
        cumsum += value
        stop = offset + index
        if cumsum == invalid and (stop - start) > 1:
            print("cumsum: ", cumsum)
            print("range: ", start, stop)
            print("min: ", min(data[start:stop]))
            print("max: ", max(data[start:stop]))
        elif cumsum > invalid:
            # reset the sum
            cumsum = value
            start = offset + index