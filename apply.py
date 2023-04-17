#!/usr/bin/python3

with open("src/tiles.rs") as f:
    content: str = f.read()

result = ''

while True:
    next_add = content.find("//ADD")
    if next_add == -1:
        break

    result += content[:next_add]
    content = content[(next_add + 5):]

result += content

with open("src/tiles.rs", "w") as f:
    f.write(result)