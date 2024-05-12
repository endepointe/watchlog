

# Documentation

## Table of Contents
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Data Types](#data-types)

### Architecture


### Configuration
To make changed to the uystemd Unit File:

Make a copy of the `/etc/systemd/system/watch-log.service` file. The following is an example of the default systemd unit file for a service that runs the watch-log service:

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


### Data Types

<table>
<tr>
<td>Config</td>
<td>Defaults</td>
<td>Log</td>
<td>Source</td>
<td>Destination</td>
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
    compression_level: Option<u8>,
    key: Option<PathBuf>,
    tx_interval: Option<String>,
}
```
</td>
<td>

```rust

struct Log {
   src: Source,
   dst: Destination,
   compression_level: Option<u8>,
   key: Option<PathBuf>,
   tx_interval: Option<String>,}
```
</td>
<td>
j
```rust

struct Source {
    name: String,
    path: PathBuf,
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

```mermaid
graph LR;
    ReadLogs-->StartWatcher;
    StartWatcher"-->Send_to_Encryptor;
    SendtoEncryptor-->SendtoCompressor;
``` 
