def preParse(line):
    line = line.replace("one", "o1e").replace("two", "t2o").replace("three", "t3e").replace("four", "f4r").replace("five", "f5e")
    line = line.replace("six", "s6x").replace("seven", "s7n").replace("eight", "e8t").replace("nine", "n9e").replace("zero", "z0o")
    # print(line)
    return line

def get(line):
    i = 0
    left = True
    right = True

    acc = 0

    while i < len(line):
        if left and 0x30 < ord(line[i]) and ord(line[i]) < 0x3A:
            acc = acc + 10 * (ord(line[i]) - 0x30)
            left = False

        if right and 0x30 < ord(line[len(line) - i - 1]) and ord(line[len(line) - i - 1]) < 0x3A:
            acc = acc + (ord(line[len(line) - i - 1]) - 0x30)
            right = False
        i = i + 1
    print(acc)
    return acc




f = open("input.txt")
lines = f.readlines()
total = 0

for l in lines:
    l = preParse(l)
    total += get(l)

print(total)
#54706


