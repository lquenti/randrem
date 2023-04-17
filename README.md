# randrem - Random Reminder

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

# LICENSE

MIT