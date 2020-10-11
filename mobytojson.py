#!/usr/bin/python

import re
import numpy as np
import json

words = {}

with open("mobypos.txt", 'r') as m:
    for line in m:
        line = line.rstrip() #Remove newline characters
        (word, pos) = re.split(r'\\', line)

        for part in pos: #Place word in every part of speech it has
            words[part] = [word] if part not in words.keys() else words[part] + [word]

with open("mobywords.json", "w") as outfile:
    json.dump(words, outfile, sort_keys = True, indent = 4, ensure_ascii = False)
