# randrem - Random Reminder

**Note: This crate is actively maintained, there is just nothing to do rn**

Randrem sends you random remainders via notification!

## Installation

```
cargo install randrem
```

## Usage

1. Define a JSON file, see [`./example.json`](./example.json) for example:

```json
[
    {
      "min_sec": 900,
      "max_sec": 3600,
      "text": "Remember to have good posture."
    },
    {
      "min_sec": 900,
      "max_sec": 3600,
      "text": "Remember to breathe through your nose."
    },
    {
      "min_sec": 3600,
      "max_sec": 7200,
      "text": "Remember to drink more water."
    }
]

```
2. Call the program with
```
randrem ./path/to/example.json
```
3. Be happy :)

## How to run it at autostart

1. Create a service unit file at `/etc/systemd/system/randrem.service`
2. Fill it accordingly. Mine currently looks like
```
[Unit]
Description=My Randrem Tool
After=network.target

[Service]
ExecStart=/home/lquenti/code/randrem/target/release/randrem /home/lquenti/code/randrem/example.json

[Install]
WantedBy=default.target
```
3. Enable and start it   
```
sudo systemctl enable randrem
sudo systemctl start randrem
```
4. Verify whether it worked
```
sudo systemctl status randrem
```


# LICENSE

MIT
