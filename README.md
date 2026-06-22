# Rust Disk Mounter

A lightweight Rust utility that automatically mounts a disk by UUID and restarts Docker containers after the disk becomes available.

If the disk is disconnected, it sends a notification using `ntfy`.

## Features

* Detects disks using their UUID
* Sends an `ntfy` notification when the disk is unavailable
* Automatically mounts the disk
* Prevents duplicate mounts
* Restarts all running Docker containers after mounting
* Uses minimal dependencies

## Requirements

Install the following tools:

* `blkid`
* `mountpoint`
* `docker`
* `curl`
* `sudo`

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

Clone the repository:

```bash
git clone https://github.com/<your-username>/rustdiskmounter.git

cd rustdiskmounter
```

Build the project:

```bash
cargo build --release
```

The binary will be available at:

```text
target/release/rustdiskmounter
```

## Configuration

Create a `.env` file.

Example:

```env
uuid=435345654645

mount=/mnt/storage

ntfy=my-ntfy-topic
```

### Variables

| Variable | Description           |
| -------- | --------------------- |
| `uuid`   | Disk UUID             |
| `mount`  | Mount point directory |
| `ntfy`   | ntfy.sh topic name    |

## Usage

Run manually:

```bash
cargo run
```

Or run the release binary:

```bash
./target/release/rustdiskmounter
```

## Workflow

1. Read configuration from `.env`
2. Check whether the disk exists
3. Send an `ntfy` notification if the disk is disconnected
4. Check if the disk is already mounted
5. Mount the disk if necessary
6. Get all running Docker containers
7. Restart the containers

## Example ntfy notification

```
disk disconnected
```

## Running automatically

You can run it with `cron` or `systemd`.

### Cron example

Run every minute:

```cron
* * * * * /path/to/rustdiskmounter
```

### Systemd example

```ini
[Unit]
Description=Rust Disk Mounter

[Service]
ExecStart=/path/to/rustdiskmounter

[Install]
WantedBy=multi-user.target
```

## License

MIT
