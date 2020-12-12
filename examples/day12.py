moves = [ line.strip() for line in open("inputs/day12.txt") ]

facing = 0 # 0: E, 1: N, 2: W, 3: S
x, y = 0, 0
for move in moves:
    direction, amount = move[0], int(move[1:])

    if direction == "N":
        y += amount
    elif direction == "S":
        y -= amount
    elif direction == "E":
        x += amount
    elif direction == "W":
        x -= amount
    elif direction == "L":
        facing += (amount / 90)
        facing %= 4
    elif direction == "R":
        facing -= (amount / 90)
        facing %= 4
    elif direction == "F":
        if facing == 0:
            x += amount
        elif facing == 1:
            y += amount
        elif facing == 2:
            x -= amount
        else:
            y -= amount

print(f"Part 1: {x}, {y} == {abs(x) + abs(y)}")

x, y = (10, 1)
ship_x, ship_y = (0, 0)
for move in moves:
    direction, amount = move[0], int(move[1:])

    if direction == "N":
        y += amount
    elif direction == "S":
        y -= amount
    elif direction == "E":
        x += amount
    elif direction == "W":
        x -= amount
    elif direction == "L":
        for i in range(amount // 90):
            x, y = -y, x
    elif direction == "R":
        for j in range(amount // 90):
            x, y = y, -x
    elif direction == "F":
        ship_x += x * amount
        ship_y += y * amount

x,y = ship_x, ship_y

print(f"Part 2: {x}, {y} == {abs(x) + abs(y)}")
