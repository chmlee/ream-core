"generate benchmark files"

import os

string = 'ream'
number = '$3.14159$'
boolean = '`FALSE`'

if not os.path.exists('./input'):
    os.makedirs('./input')
if not os.path.exists('./output'):
    os.makedirs('./output')

def generateFile(path, entryList, varList):
    print(f"generating {path}")
    # init
    if os.path.exists(path):
        os.remove(path)
    os.mknod(path)
    file = open(path, "w")
    file.writelines("# Dataset\n")

    def addEntry(file, i, entryList=entryList, varList=varList):
        level = i + 2
        entryCount = entryList[i]
        stringCount, numberCount, booleanCount = varList[i]
        for j in range(entryCount):
            file.write(f"{'#'*level} Level_{level}\n")
            for j1 in range(stringCount):
                file.writelines(f"- string_{j1}: {string}\n")
            for j2 in range(numberCount):
                file.writelines(f"- number_{j2}: {number}\n")
            for j3 in range(booleanCount):
                file.writelines(f"- boolean_{j3}: {boolean}\n")
            i += 1
            if i <= len(entryList) - 1:
                addEntry(file, i)
                i -= 1
    addEntry(file, i=0)


generateFile("./input/b1.md", [10000], [[120, 120, 120] for _ in range(1)])
generateFile("./input/b2-1.md", [1000, 10], [[60, 60, 60] for _ in range(2)])
generateFile("./input/b2-2.md", [100, 100], [[60, 60, 60] for _ in range(2)])
generateFile("./input/b2-3.md", [10, 1000], [[60, 60, 60] for _ in range(2)])
generateFile("./input/b3-1.md", [100, 10, 10], [[40, 40, 40] for _ in range(3)])
generateFile("./input/b3-2.md", [10, 100, 10], [[40, 40, 40] for _ in range(3)])
generateFile("./input/b3-3.md", [10, 10, 100], [[40, 40, 40] for _ in range(3)])
generateFile("./input/b4.md", [10, 10, 10, 10], [[30, 30, 30] for _ in range(4)])
