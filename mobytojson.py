#!/usr/bin/python

import re
import numpy as np
import json

words = []

with open("moby.txt", 'r') as m:
    m.readline()
    for line in m:
        #line.encode('utf-8')

        (word, pos) = re.split(r'\\', line)
        arr = [word, pos] 
        words.append(arr)

json_words = json.dumps(words)    

with open("mobywords.json", "w") as outfile:
    json.dump(json_words, outfile)
