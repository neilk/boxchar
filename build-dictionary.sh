#!/bin/bash

set -e 

cargo run --bin dictionary-builder -- --frequencies data/google-ngrams-words-all.txt > /tmp/wordlist.txt
sort -k 2,2rn -k 1 /tmp/wordlist.txt > data/wordlist.txt