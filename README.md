#
System Event Management Software should allow users to capture system and application activity while owning their data. By collecting system and application events using a simple and cost-efficient solution, users can monitor the health of their digital assets.


### Systemd Unit File:
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
