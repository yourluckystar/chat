## chat

### Installation & Usage

#### Linux
```bash
sh -c "$(curl -sS https://raw.githubusercontent.com/7791372/chat/main/install.sh)"
```

#### Building from Source
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Clone the repository
git clone https://github.com/7791372/chat.git
cd chat

# Build and run the client
cargo build --release --bin client
./target/release/client

# Build and run the server (Only if you are going to be hosting)
cargo build --release --bin server
./target/release/server
```

#

### Server setup
###### Currently the project is designed to work with `SSH Tunneling` only

<b>Port-forwarding</b><br>
<sup>Open port 22 (ssh) in your router settings located by going to [192.168.1.1](https://192.168.1.1) in your browser.</sup>

<b>Configure your firewall</b><br>
<sup>To enable incoming/outgoing ssh connections, you have to allow them in your firewall. In this example I'll use `ufw (uncomplicated firewall)`</sup>

```bash
sudo pacman -S ufw (for ArchLinux)

sudo ufw allow ssh
sudo ufw enable
sudo systemctl enable ufw
```

<b>Start the server</b><br>
<sup>By default, the server opens on `127.0.0.1:11945`</sup>

```bash
# Connecting using SSH Tunneling
ssh -NL 11945:localhost:11945 user@ip

where user is the profile name of the PC you are connecting to
and ip is the public IP address of the host
```

---

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.