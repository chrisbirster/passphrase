# passphrase

Passphrase is a simple binary application that will generate a random "dice roll" and create a 6 word passphrase. The wordlist is taken from ![Electronic Frontier Fondation](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt). There is also a blog plost by [EFF](https://www.eff.org/dice) on why you want to use a passphrase. 


The application is built using the latest stable Rust release v1.50.0. The instructions assume a working installation of Rust. 

## Build
```
$ git clone git@github.com:chrisbirster/passphrase.git
$ cd passphrase
$ cargo build 
```

## Test
```
$ cd passphrase
$ cargo test
```
