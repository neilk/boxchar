Scoring:
- still many paradoxes. "DOJO-OVERHYPING" is more pleasing than "DRIVE-EPINEPHRINE-ENJOYING". 
-- we could simply rank all 2-words better than 3-words
-- optimizing for length seems to be less good
-- ephinephrine seems to be unusually common in the ngrams corpus. .

- when searching for solutions, cut off the dictionary iteration to a shorter number, if/when we are going deeper.
    - since we have sorted all of this, we can cache those sub-dictionaries somehow as simple integers, the end index to stop at.
    - that suggests that perhaps we should actually print out the dictionary with boundaries for frequencies rather than one frequency per line!
