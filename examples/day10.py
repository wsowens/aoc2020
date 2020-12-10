import functools
from collections import defaultdict

data = []
for line in open("inputs/day10.txt"):
    data.append(int(line.strip()))

# add initial charging port
data.append(0)
data.sort()
# add final phone
data.append(data[-1] + 3)

# part 1
counts = defaultdict(int)
for (prev, n) in zip(data[:-1], data[1:]):
    counts[n-prev] += 1
print(counts)
jolt3 = counts[3]
jolt1 = counts[1]
print(f"{jolt3} x {jolt1} = {jolt3 * jolt1}")


# part 2
# a simple, memoized function
@functools.lru_cache(maxsize=None)
def count_adapter(*adapters):
    if len(adapters) == 1 or not adapters:
        return 1
    current = adapters[0]
    count = 0
    for i, nxt in enumerate(adapters):
        if (nxt - current) in range(1,4):
            count += count_adapter(*adapters[i:])
        # we're out of range, no point checking any more
        elif nxt > current:
            break
    return count

#print(count_adapter(*data))
data = tuple(data)

from timeit import timeit


def time_func(func, count):
    time = timeit(func, number=count)
    print(f"time: {time} / {count} ({time/count} per iteration)")
    return func()


@functools.lru_cache(maxsize=None)
def count_adapter2(start, adapters):
    if start >= len(adapters) - 1:
        return 1
    current = adapters[start]
    count = 0
    for i, nxt in enumerate(adapters[start:]):
        if (nxt - current) in range(1,4):
            count += count_adapter2(start + i, adapters)
        # we're out of range, no point checking any more
        elif nxt > current:
            break
    return count

# conclusion: these functions are basically the same speed
# no matter how you slice it, the tuple will get hashed
result = time_func(lambda: count_adapter(*data), 10000)
print("count_adapter:", result)
result2 = time_func(lambda: count_adapter2(0, data), 10000)
print("count_adapter2:", result2)
