#!/bin/bash
# Encrypts and compresses a list of files with a given extension: .tex
rm *.aux *.log *.out *.pdf

files=$(ls *$1)
read -sp "Enter password: " password
echo
read -sp "Confirm password: " password2
echo
if [[ $password != $password2 ]]; then
    echo "Passwords do not match. Exiting."
    exit 1
fi

for file in $files; do
    if [[ $file == *.tex ]]; then
        echo "Encrypting $file."
        openssl enc -aes256 -pbkdf2 -in "$file" -out "$file.aes256" -k "$password"
        while ps -p $! &>/dev/null; do
            echo -n "."
            sleep 1
        done
        echo "Encrypted file $file. Now compressing."
        gzip $file.aes256
        while ps -p $! &>/dev/null; do
            echo -n "."
            sleep 1
        done
    fi
done

rm *.tex
