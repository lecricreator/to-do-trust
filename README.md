``` 
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                         │
│  TTTTTTT  OOOOO         DDDDD    OOOOO         TTTTTTT RRRRRR  UU   UU  SSSSS  TTTTTTT  │
│    TTT   OO   OO        DD  DD  OO   OO          TTT   RR   RR UU   UU SS        TTT    │
│    TTT   OO   OO _____  DD   DD OO   OO _____    TTT   RRRRRR  UU   UU  SSSSS    TTT    │
│    TTT   OO   OO        DD   DD OO   OO          TTT   RR  RR  UU   UU      SS   TTT    │
│    TTT    OOOO0         DDDDDD   OOOO0           TTT   RR   RR  UUUUU   SSSSS    TTT    │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```
# to-do-listing
Create my own to-do-list in Rust for learnig Rust.


# Installation
### Start to install cargo for compile language Rust.<br/>
On Linux terminal
```sh
curl https://sh.rustup.rs -sSf | sh
``` 
On windows
On Windows, download and run [rustup-init.exe]()

or other
[More information for install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

On linux terminal copy this command<br/>
```
mkdir $HOME/todotrust
git clone https://github.com/lecricreator/to-do-trust.git "$HOME/todotrust"
cd $HOME/todotrust
cargo build --release
cp ./target/release/todotrust .
```
Make shortcut for use when you want.
Verified if your on bash or zsh with this command :
```
echo $SHELL
```
On Bash :
```
printf 'alias todotrust="$HOME/todotrust/todotrust"' >> "$HOME/.bashrc"
source ~/.bashrc
```
On zsh :
```
printf 'alias todotrust="$HOME/todotrust/todotrust"' >> "$HOME/.zshrc"
source ~/.zshrc
```