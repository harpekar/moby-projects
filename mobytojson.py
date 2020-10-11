#!/usr/bin/python

import re
import numpy as np
import json

pos_legend = { #The legend given in the original Moby file for each part of speech
    'N' : 'Noun',
    'P' : 'Plural',
    'h' : 'Noun Phrase',
    'V' : 'Verb (usu participle)',
    't' : 'Verb (transitive)',
    'i' : 'Verb (intransitive)',
    'A' : 'Adjective',
    'v' : 'Adverb',
    'C' : 'Conjunction',
    'P' : 'Preposition',
    '!' : 'Interjection',
    'r' : 'Pronoun',
    'D' : 'Definite Article',
    'I' : 'Indefinite Article',
    'o' : 'Nominative'
}


words = {}

with open("mobypos.txt", 'r') as m:
    for line in m:
        line = line.rstrip() #Remove newline characters
        (word, pos) = re.split(r'\\', line)

        for part in pos: #Place word in the dict of every part of speech it has
            full_pos = pos_legend.get(part)
            words[full_pos] = [word] if full_pos not in words.keys() else words[full_pos] + [word]


with open("mobywords.json", "w") as outfile:
    json.dump(words, outfile, sort_keys = True, indent = 4, ensure_ascii = False)
