#!/usr/bin/python

import re

words = {}

with open("mobypos.txt", 'r') as m:
    m.readline()
    for line in m:
        print(line)
        (word, pos) = re.split(r'\\', line)
        words[word] = pos
        print(word)


print(words['A1'])
