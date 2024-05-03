#!/bin/bash
# Decompresses and decrypts a list of files with a given extension: .gz and .aes256.
read -sp "Enter password: " password
echo

files=$(ls)

for file in $files; do
    if [[ $file == *.gz ]]; then
        echo "Decompressing $file."
        gzip -d $file
        while ps -p $! &>/dev/null; do
            echo -n "."
            sleep 1
        done
    fi
done

files=$(ls)

for file in $files; do
    if [[ $file == *.aes256 ]]; then
        echo "Decrypting $file."
        decfile=$(echo $file | sed 's/.aes256//')
        echo $decfile
        openssl enc -aes256 -pbkdf2 -d -in "$file" -out "$decfile" -pass pass:"$password"
        while ps -p $! &>/dev/null; do
            echo -n "."
            sleep 1
        done
    fi
done

rm *.aes256
