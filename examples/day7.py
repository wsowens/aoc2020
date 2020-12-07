import re
from pprint import pprint

RULE_RE = re.compile("(\d \w+ \w+) bags?")

data = open("inputs/day7.txt").read()
rules = data.split("\n")
rules = [ rule.replace("contain ", "") for rule in rules ]


def parse_contains(rule):
    matches = RULE_RE.findall(rule)
    matches = [match.split() for match in matches]
    return [ (" ".join(match[1:]), int(match[0])) for match in matches ]

rules = {
    " ".join(rule.split()[:2]) : parse_contains(rule) for rule in rules
}

def look_inside(bag):

    contains = [ inside_bag for (inside_bag, _) in rules[bag] ]
    if "shiny gold" in contains:
        return True
    else:
        return any([look_inside(bag) for bag in contains])

def count_inside(bag):
    count = 0
    for (b, c) in rules[bag]:
        count += c
        count += c * count_inside(b)
    return count


count = 0
for bag in rules:
    if look_inside(bag):
        count += 1

print(count)
print(count_inside("shiny gold"))