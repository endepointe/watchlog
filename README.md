


# Watch-Log

## Table of Contents:

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)

### Overview:
System Event Management Software should allow users to capture system and application activity while owning their data. By collecting system and application events using a simple and cost-efficient solution, users can monitor the health of their digital assets.

### Installation:
(not ready) curl --proto '=https' --tlsv1.2 -sSf https://sh.endepointe.com | bash

### Usage:
After installation, start the service using the following command:

systemctl enable watch-log.service
systemctl start watch-log.service

To check the status of the service, use the following command:

systemctl status watch-log.service

Good to go.

```bash

Systemd Unit File:

To make changes to the default systemd unit file, make a copy of the `/etc/systemd/system/watch-log.service` file. The following is an example of the default systemd unit file for a service that runs the watch-log service:


```bash

[Unit]
Description=Watch-Log Service

[Service]
ExecStart=bin/bash -c 'path/to/watch-log > path/to/watch-log-health 2>&1'
WorkingDirectory=path/to
StandardOutput=journal

[Install]
WantedBy=multi-user.target
```
