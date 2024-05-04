#!/bin/bash

help() {
    echo "Usage: $0 <file>"
    echo "Encrypts a file using the aes256 cipher"
}
main() {

    if [ $# -ne 1 ]; then
        help
        exit 1
    fi

    file=$1

    read -sp "Enter password: " password
    echo
    read -sp "Re-enter password: " password2

    if [[ $password != $password2 ]]; then
        echo "Passwords do not match"
        exit 1
    fi
    
    openssl enc -aes256 -pbkdf2 -in "$file" -out "$file.aes256" -k "$password"

    while ps -p $! &>/dev/null; do
        echo -n "."
        sleep 1
    done

    echo "Encryption complete"
    
    #Compressing an already small file is not worth it. What is 
    #that threshold? Find out.
    #echo "Compressing $file.aes256"
    #gzip "$file.aes256"

    rm "$file"
}

main $@ || exit 1
