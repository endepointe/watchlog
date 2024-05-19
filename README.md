


# WatchLog

# Table of Contents:

- [Overview](#overview)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Docs](#docs)

## Overview:
System Event Management Software should allow users to capture system and application activity while owning their data. By collecting system and application events using a simple and cost-efficient solution, users can monitor the health of their digital assets.

## Installation:
(not ready) curl --proto '=https' --tlsv1.2 -sSf https://sh.endepointe.com | bash

## Configuration:
To set the log collection buffer, set the ```tx_buffer``` value in `config.json` to one of the following:

Two column table:
| Interval | Description |
| --- | --- |
| 1kb | 1024 bytes |
| 4kb | 4096 bytes |
| 1mb | 1024 kilobytes |
| stream | Stream the logs |

The default value is 1kb.


## Usage:
After installation, start the service using the following command:

systemctl enable watchlog.service
systemctl start watchlog.service

To check the status of the service, use the following command:

systemctl status watchlog.service

Good to go.



### Docs:
For more information, please refer to the [docs](./docs/README.md)
