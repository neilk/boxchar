Now that we have frequency scores: 

- implement sorting by score
- when searching for solutions, cut off the dictionary iteration to a shorter number, if/when we are going deeper.
    - since we have sorted all of this, we can cache those sub-dictionaries somehow as simple integers, the end index to stop at.
    - that suggests that perhaps we should actually print out the dictionary with boundaries for frequencies rather than one frequency per line!



- cache the parsed dictionary in wasm-land
- we're creating it every single time!!!

