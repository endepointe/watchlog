#!/bin/bash

# install the events of the application in /var/log/watch-log/events.log
create_events_log() {
    if [ ! -d /var/log/watch-log ]; then
        mkdir -p /var/log/watch-log/logs
        touch /var/log/watch-log/logs/events.log
    fi
}

# create a file called `watch-log.service`and copy the file into /etc/systemd/system/
move_service_file() {
    cp watch-log.service /etc/systemd/system/
}

# make a hash of the file and write it to checksum.txt
make_hash() {
    sha256sum watch-log.service > checksum.txt 
}

main() {
    echo "main install script"
    echo "$@"
}

main "$@" || exit 1
