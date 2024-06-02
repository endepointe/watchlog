

# WatchLog

# Table of Contents:

- [Overview](#overview)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Architecture](#architecture)
- [Data Types](#data-types)

## Overview:
System Event Management Software should allow users to capture system and application activity while owning their data. By collecting system and application events using a simple and cost-efficient solution, users can monitor the health of their digital assets.

## Installation:

### Prebuilds
(not ready) curl --proto '=https' --tlsv1.2 -sSf https://sh.endepointe.com | bash

### Building from source:
To build from source, clone the repository and run the following commands:

```bash
git clone https://github.com/endepointe/watchlog.git
cd watchlog/app
cargo build --bin watchlog --release
```

## Configuration:

### Configuring the client:

To enable the client to run on startup, add the following to your <a href="#Systemd-Unit-File">systemd unit file</a>:

```bash
ExecStart=bin/bash -c 'path/to/watchlog/app/target/release/watchlog'
```

### Configuring the server:

Install the storage-controller binary onto the server machine:

```bash
git clone https://github.com/endepointe/watchlog.git
cd watchlog/app
cargo build --bin storage-controller --release
```

To enable the server to run on startup, add the following to your <a href="#Systemd-Unit-File">systemd unit file</a>:


```bash
ExecStart=bin/bash -c 'path/to/watchlog/app/target/release/storage-controller'
```

### Setting up the logs:

To set up the logs to be collected, add desired log name(s) and path(s) to the `config.json` file:

```json
{
    "logs": [
        {
            "src": {
                "name": "<log_name>",
                "path": "path/to/log"
            },
            "dst": {
                "address": "..."
                "port": "..."
            },
            ...
            ...
            ...
        }
    ]
}
```

### Generate public and private keys:

```bash
cd watchlog/app
cargo build --bin generate --release
./target/release/generate <private_key_file> <public_key_file>
```
### Setting transmission buffer size:
To set the log collection buffer, set the ```tx_buffer``` value in `config.json` to one of the following:

| Interval | Description |
| --- | --- |
| 1KB | 1024 bytes |
| 4KB | 4096 bytes |
| 1MB | 1024 kilobytes |
| stream | Stream the logs |

The default value is 1KB.


### Systemd Unit File:

To make changes to the systemd unit file (Client and Server):

Make a copy of the `/etc/systemd/system/watchlog.service` file. The following is an example of the default systemd unit file for a service that runs the watchlog service:


#### Client:
```bash
[Unit]
Description=WatchLog Service - WatchLog

[Service]
ExecStart=bin/bash -c 'path/to/watchlog'
WorkingDirectory=path/to
StandardOutput=journal

[Install]
WantedBy=multi-user.target
```

#### Server:
```bash
[Unit]
Description=WatchLog Service - Storage Controller

[Service]
ExecStart=bin/bash -c 'path/to/storage-controller'
WorkingDirectory=path/to
StandardOutput=journal

[Install]
WantedBy=multi-user.target
```

## Usage:
After installation and configuration, start the service using the following command:

```bash
systemctl enable watchlog.service
systemctl start watchlog.service
```

To check the status of the service, use the following command:

```bash
systemctl status watchlog.service
```

To run the client without systemd, use the following command:

```bash
'./path/to/watchlog/app/target/release/watchlog'

//or

cargo run --bin watchlog 
```

To run the server without systemd, use the following command:

```bash
'./path/to/watchlog/app/target/release/storage-controller'

//or

cargo run --bin storage-controller
```

Good to go.


## Architecture

#### Client

```mermaid
graph LR 
A(ReadConfig) --> L1(path/to/syslog)
A --> L2(path/to/auth.log)
A --> L3(path/to/messages)
A --> L4(path/to/secure)
L1 --> W(WatchLog)
L2 --> W
L3 --> W
L4 --> W
W --> Col(Collector)
Col --> W
Col --> Enc(Encrypt)
Enc --> Cmpr(Compress)
Cmpr --> Tx(Transmit)
Tx --> |purge| Col 
Tx --> S[(StorageController)]
``` 

#### Server
```mermaid
graph LR
S[(StorageController)] --> l1(logs/auth/)
S --> l2(logs/syslog/)
S --> l3(logs/messages/)
S --> l4(logs/secure/)
l1 --> h1(yyyy-mm-dd/hh-mm)
l1 --> h2(yyyy-mm-dd/hh-mm)
l2 --> h3(yyyy-mm-dd/hh-mm)
l2 --> h4(yyyy-mm-dd/hh-mm)
l3 --> h5(yyyy-mm-dd/hh-mm)
l3 --> h6(yyyy-mm-dd/hh-mm)
l4 --> h7(yyyy-mm-dd/hh-mm)
l4 --> h8(yyyy-mm-dd/hh-mm)
```

### Data Types

<table>
<tr>
<td>Config</td>
<td>Defaults</td>
<td>Log</td>
</tr>
<tr>
<td>

```rust

struct Config {
    logs: Vec<Log>,
    defaults: Defaults,
}
```
</td>
<td>

```rust

struct Defaults {
    compression_level: u8,
    key: String,
    tx_buffer: String,
}
```
</td>
<td>

```rust

struct Log {
   src: Source,
   dst: Destination,
   compression_level: Option<u8>,
   key: Option<String>,
   tx_buffer: Option<String>,
```
</td>
</tr>
<tr>
<td>Header</td>
<td>Source</td>
<td>Destination</td>
</tr>
<tr>
<td>

```rust

struct Header {
    name: String,
    date: String,
}
```
</td>
<td>

```rust

struct Source {
    name: String,
    path: String,
}
```
</td>
<td>

```rust

struct Destination {
    address: IpV4Addr or IpV6Addr,
    port: u16,
}
```
</td>
</tr>
</table>

## License
GNU General Public License v3.0

### <a href="#Table-of-Contents">Back to top</a>

