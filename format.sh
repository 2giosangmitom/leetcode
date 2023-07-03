#!/bin/bash

echo -e "\033[1;32m-------------------\033[0m"

echo -e "\033[0;36mFormatting TypeScript...\n\033[0m"
prettier --write TypeScript/*.ts

echo -e "\n\033[0;36mFormatting Java...\n\033[0m"
java -jar javaformat.jar --replace Java/*.java

echo -e "\033[1;32mDone!\033[0m"
