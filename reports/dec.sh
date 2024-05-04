#!/bin/bash

help() {
    echo "Usage: $0 <file>"
    echo "Decrypts a file using the aes256 cipher"
}
main() {

    if [ $# -ne 1 ]; then
        help
        exit 1
    fi

    file=$1

    read -sp "Enter password: " password
    echo 

    decfile=$(echo $file | sed 's/.aes256//')
    echo "Decrypting $file to $decfile"
    openssl enc -d -aes256 -pbkdf2 -in "$file" -out "$decfile" -pass pass:"$password"

    while ps -p $! &>/dev/null; do
        echo -n "."
        sleep 1
    done

    echo "Decryption complete"
    rm "$file"
}

main $@ || exit 1
