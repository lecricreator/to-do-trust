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
An simple to do list for your shopping and your session code.

<img src="https://github.com/user-attachments/assets/69ed8762-ef67-4e45-888b-2a5f19ed9e87" alt="gif show functionnality of todotrust" title="show todotrust" width="900"/>

# Installation
### Start to install cargo for compile language Rust.<br/>
On Linux terminal
```sh
curl https://sh.rustup.rs -sSf | sh
```

On linux terminal copy this command<br/>
```
mkdir $HOME/todotrust
git clone https://github.com/lecricreator/to-do-trust.git "$HOME/todotrust"
cd $HOME/todotrust
cargo build --release
cp ./target/release/todotrust .
```
### Make a shortcut<br/>
Verified if your on bash or zsh with this command :
```
echo $SHELL
```
If you're on Bash :
```
echo 'alias todotrust="$HOME/todotrust/todotrust"' >> "$HOME/.bashrc"
echo 'export TODORUST_FILE="$HOME/todotrust"' >> "$HOME/.bashrc"
source ~/.bashrc
```
If you're on zsh :
```
echo 'alias todotrust="$HOME/todotrust/todotrust"' >> "$HOME/.zshrc"
echo 'export TODORUST_FILE="$HOME/todotrust"' >> "$HOME/.zshrc"
source ~/.zshrc
```
# use of AI
- This project was entirely created by **Lecricreator**.<br/>
- The creator asked at ai only **one question per day**.
- The only use was to understand Rust concepts.
- This project **never copied AI-generated code** and always wrote it **himself**.

# Personal experience <br />
### **First project create in Rust.**<br />
The goal was to terminate a project in Rust for see all the process and the mentality.<br />
I learn :
- managing file (open, write, read).
- rename (replace file).
- managing error and use lib thisError for more possibility
- use lib color for have color in the terminal.

<br/>
Total hour of work on todotrust : ~50h