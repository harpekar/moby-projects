#!/usr/bin/python

import re
import numpy as np
import json

words = {}

with open("mobypos.txt", 'r') as m:
    for line in m:
        line = line.rstrip() #Remove newline characters
        (word, pos) = re.split(r'\\', line)
        words[word] = pos

with open("mobywords.json", "w") as outfile:
    json.dump(words, outfile, sort_keys = True, indent = 4, ensure_ascii = False)
