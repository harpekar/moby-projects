#!/usr/bin/python

import re
import numpy as np
import json

words = {}

with open("mobypos.txt", 'r') as m:
    for line in m:
        line = line.rstrip()
        (word, pos) = re.split(r'\\', line)
        words[word] = pos

with open("mobywords.json", "w") as outfile:
    json.dump(words, outfile)
