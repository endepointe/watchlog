

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
![Data Types](./data-types.png)
```rust
struct Log {
   src: Source,
   dst: Destination,
   compression_level: u8,
   key: Path,
   tx_interval: &str,
}

struct Source {
    name: &str,
    path: Path,
}

struct Destination {
    address: IpV4Addr | IpV6Addr,
    port: u16,
}
```

| Log  | 
| ------------- |
| src: Source|
| dst: Destination |
| compression_level: u8 |
| key: Path, |
| tx_interval: &str |

<table>
  <tr>
    <td>Status</td>
    <td>Response</td>
  </tr>
  <tr>
    <td>200</td>
    <td>

```rust

struct Log {
   src: Source,
   dst: Destination,
   compression_level: u8,
   key: Path,
   tx_interval: &str,
}
```

    </td>
  </tr>
  <tr>
    <td>400</td>
    <td>
      **Markdown** _here_. <!-- Blank lines above and below! -->
    </td>
  </tr>
</table>

