[![Build Status](https://travis-ci.org/safex/vote.png?branch=master)](https://travis-ci.org/safex/vote)
This is still under development, and should be treated as testing.

##### Install Rust Ubuntu 14.04, 15.04, 15.10

```bash
# install rust stable
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh

# install stable and make it default
sudo multirust update stable
sudo multirust default stable
```
##### Install Rust OSX with Homebrew

```bash
# install multirust
brew update
brew install multirust

# install stable and make it default
multirust update stable && multirust default stable
```
#### Validate a poll with votes in same directory

```bash
# download and build safex/vote
git clone https://github.com/safex/vote
cd vote
cargo run --bin validate
```

#### Forming a poll

```bash
# download and build safex/vote
git clone https://github.com/safex/vote
cd vote
cargo run --bin poll
```

#### Voting on a poll

```bash
# download and build safex/vote
git clone https://github.com/safex/vote
cd vote
cargo run --bin vote
```


#### Generate random base64 && base58(WIF) private bitcoin and public key pair

```bash
# download and build safex/vote
git clone https://github.com/safex/vote
cd vote
cargo run --bin keys
```
