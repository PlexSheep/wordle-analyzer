import json

with open('../data/wordlists/german_SUBTLEX-DE_small.json', 'r') as f:
    word_freqs: dict[str,float] = json.load(f)
    nd: dict[str, float] = dict()
    for k in word_freqs:
        nd[k.lower()] = word_freqs[k]

with open('../data/wordlists/german_SUBTLEX-DE_small.json', 'w') as f:
    json.dump(nd, f, indent=4)
