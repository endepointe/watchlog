


# Watch-Log

## Table of Contents:

- [Overview](#overview)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Docs](#docs)

### Overview:
System Event Management Software should allow users to capture system and application activity while owning their data. By collecting system and application events using a simple and cost-efficient solution, users can monitor the health of their digital assets.

### Installation:
(not ready) curl --proto '=https' --tlsv1.2 -sSf https://sh.endepointe.com | bash

### Configuration:
To set the log collection interval, set the ```tx_interval``` value in `config.json` to one of the following:

Two column table:
| Interval | Description |
| --- | --- |
| 1m | 1 minute |
| 5m | 5 minutes |
| 10m | 10 minutes |
| 30m | 30 minutes |
| 1h | 1 hour |
| 6h | 6 hours |
| 12h | 12 hours |

The default value is 1m.


### Usage:
After installation, start the service using the following command:

systemctl enable watch-log.service
systemctl start watch-log.service

To check the status of the service, use the following command:

systemctl status watch-log.service

Good to go.



### Docs:
For more information, please refer to the [docs](./docs/README.md)
