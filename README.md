# randrem - Random Reminder

randrem is a set of shell scripts that sends random remainders. This can for example be used to have a better posture or drink more water while being in the zone

## Dependencies

The randrem scripts require the `notify-send` utility to be installed on your system.

- Ubuntu: `sudo apt-get install libnotify-bin`
- Arch: `sudo pacman -S libnotify`
- Fedora: `sudo dnf install libnotify`

## Getting Started

To use randrem, you'll need to clone the repository and edit the `call.sh` script to customize the reminder messages and time intervals to your liking. 

Here's how to get started:

1. Clone the repository: `git clone https://github.com/<your-username>/randrem.git`
2. Edit the `call.sh` script
   - Customize the reminder messages and time intervals in the `randrem.sh` commands to fit your needs.
3. Make the scripts executable: `chmod +x randrem/call.sh randrem/randrem.sh`
4. Start the reminder thread: `cd randrem && ./call.sh`

The reminder thread will continue running until you stop it manually by pressing `Ctrl+C` in the terminal.

