# learning-near

A repository for learning near from zero experience.

## Prepare development environment for near

### Install Rust

For compiling the client of NEAR network - `nearcore`, you need to install Rust first. You can do this by simply following [the official guide of rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Clone and compile nearcore

Clone [nearcore](https://github.com/near/nearcore) to your local workspace.

Change to a branch with a stable version rather than using master branch directly. For example, we can simply checkout branch 1.19.0.

```shell
cd ~/workspace
git clone https://github.com/near/nearcore.git
cd nearcore
git checkout 1.19.0
```

Then, you can use `make release` or `make debug` command to compile nearcore. This may take couple of minutes, based on the performance of hardware of your computer.

### Install nearup

Follow the README of [nearup repository](https://github.com/near/nearup) to install `nearup` to your computer.

Or simply run the following command for the first time:

```shell
pip3 install --upgrade pip
pip3 install --user nearup
```

Check whether `nearup` is in PATH by command `which nearup`. If the response is `nearup not found`, you can add following contents to `~/.profile` or `~/.bashrc`:

```shell
USER_BASE_BIN=$(python3 -m site --user-base)/bin
export PATH="$USER_BASE_BIN:$PATH"
```

Use `source` command to run the profile of current user to make it effective for current session, then you can use `nearup` directly.

### Install near-cli

Follow the README of [near-cli repository](https://github.com/near/near-cli) to install `near-cli` to your computer.
