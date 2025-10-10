#!/bin/zsh

while read line; do 
    if [[ "$line" =~ "(--color-[a-z-]+):[[:space:]]*([^;]+)" ]]; then 
      echo "${match[1]}: $(./endarken-color.js ${match[2]});"; 
    fi
done < <(grep -- --color src/app.css)

