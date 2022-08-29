# Running `camo-rs` with `systemd`

Running `camo-rs` on `systemd` is relatively easy. For reference, here is a service file you can use:

```
[Unit]
Description=an HTTP proxy for assets (mainly images) to route requests through an always-encrypted connection

[Service]
User=nobody
EnvironmentFile=/etc/camo
ExecStart=/usr/bin/camo

[Install]
WantedBy=multi-user.target
```

Replace the binary location, and the location of the `EnvironmentFile` as needed. For all available and required environment variables, check [the documentation about configuring `camo-rs`](/docs/configuration.md).
