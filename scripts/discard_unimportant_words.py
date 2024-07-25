import json

# Load the word frequency dictionary
with open('../data/wordlists/german_SUBTLEX-DE_full.json', 'r') as f:
    word_freqs = json.load(f)

# Set a frequency threshold (e.g., 0.001)
freq_threshold = 0.000001

# Set a maximum word length (e.g., 10)
max_word_length = 10

# Filter out words with low frequency and long length
filtered_word_freqs = {word: freq for word, freq in word_freqs.items() if freq >= freq_threshold and len(word) <= max_word_length}

# Save the filtered word frequencies to a new JSON file
with open('../data/wordlists/german_SUBTLEX-DE_small.json', 'w') as f:
    json.dump(filtered_word_freqs, f, indent=4)
