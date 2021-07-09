# learning-near

A repository for learning near from zero experience.

## Prepare development environment for near

### Install Rust

For compiling the client of NEAR network - `nearcore`, or writing your own smart contracts of NEAR, you need to install Rust first. You can do this by simply following [the official guide of rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Clone and compile nearcore

Clone [nearcore](https://github.com/near/nearcore) to your local workspace.

Change to a branch with a stable version rather than using master branch directly. For example, we use stable version 1.19.1 here:

```shell
cd ~/workspace
git clone https://github.com/near/nearcore.git
cd nearcore
git checkout tags/1.19.1 -b v1.19.1
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

## Try out basic operations of NEAR by using cli

### Start a local node for testnet

You can start a local node of NEAR by `nearup` directly:

```shell
nearup run testnet --binary-path ~/workspace/nearcore/target/release
```

The initial sync up may take couple of hours, and it will download the genesis.json (up to 6GB) of testnet. If something goes wrong (like failed in downloading genesis.json, you will see errors using command `nearup logs`), you need to stop current node instance first, then delete the folder of testnet in near home and retry the above command to start up node of testnet again, for example:

```shell
nearup stop
rm -rf ~/.near/testnet
nearup run testnet --binary-path ~/workspace/nearcore/target/release
```

Or you can use scripts to start the local node in docker (using parameter 1 to specify network id):

```shell
./scripts/start_near.sh {testnet|mainnet}
```

and then checking the status by using:

```shell
docker exec nearup nearup logs --follow
```

> The `near-cli` will use default endpoint of each network. For example, the `testnet` will use `https://rpc.testnet.near.org` as node url. So you can skip this step if it's unnecessary to allow `near-cli` to interact with NEAR network through local node.

### Switch network by setting NEAR_ENV

You can use the following command to switch network environment which the cli will use.

```shell
export NEAR_ENV={network name}
```

The available `network name` inclouding:

* `mainnet`
* `development` and `testnet`
* `betanet`
* `local`
* `test` and `ci`
* `ci-betanet`

You can also check [config.js](https://github.com/near/near-cli/blob/master/config.js) of `near-cli` for more detail.

In following operations, we'll use `testnet`, which is also the default setting of NEAR_ENV.

```shell
export NEAR_ENV=testnet
```

### Create your first top level account

You need to use the [Near wallet for testnet](https://wallet.testnet.near.org) to create your first top level account on testnet. Please follow the instructions of the official wallet.

> For the concept of account in NEAR, you can refer to [the official docs](https://docs.near.org/docs/concepts/account) for detail.

### Login using cli

The so-called `login` is to let `near-cli` to manage the private key(s) of an top level account and its all subaccount(s).

You can simply run `near login` to perform the login.

> This command will automatically call up your default browser to open the wallet page of testnet and let you to select an account to be authorized. The command will automatically complete after you activated an account on the wallet page.
>
> The top level account created by wallet of testnet will holding 200 NEAR by default.

If the command succeeded, you can find a json file named by your top level account in the folder `~/.near-credentials/testnet`. And the `near-cli` will use this account silently when you perform those commands which need to use the private key(s) you have authorized.

### Check the state of network and your account

The following command(s) can check the state of testnet and your account.

```shell
near validators current
near validators next
near proposals
```

```shell
near state {your account name}.testnet
near keys {your account name}.testnet
```

### Create subaccount(s)

Now, let's create our first subaccount `sub-acct1.{your account name}.testnet`:

```shell
near create-account sub-acct1.{your account name}.testnet --masterAccount {your account name}.testnet --initialBalance 30
```

If the command succeeded, a new keyfile of the subaccount with balance 30 can be found in `~/.near-credentials/testnet/sub-acct1.{your account name}.testnet.json`.

Check the state of new subaccount by using:

```shell
near state sub-acct1.{your account name}.testnet
near keys sub-acct1.{your account name}.testnet
```

### Perform the first transaction

To complete a transaction, we need at least two accounts, so we can simply create another subaccount `sub-acct2.{your account name}.testnet`

```shell
near create-account sub-acct2.{your account name}.testnet --masterAccount {your account name}.testnet --initialBalance 25
```

Now we can try to perform the first transaction by using the 2 subaccounts we just created. For example, if we want to send 1 NEAR from subaccount `sub-acct1` to subaccount `sub-acct2`, we can run command:

```shell
near send sub-acct1.{your account name}.testnet sub-acct2.{your account name}.testnet 1
```

If the command succeeded, we'll get the id of the transaction. Then we can check the transaction by using:

```shell
near tx-status {transaction id} --accountId {account id of sender}
```

And we can see the detail of the transaction. For example:

```shell
{
  status: { SuccessValue: '' },
  transaction: {
    signer_id: 'sub-acct1.my-account.testnet',
    public_key: 'ed25519:J6Q814wc4sjdiEktvNbUGazww5Cwx1dvoUQVBntoZ5gB',
    nonce: 52327147000001,
    receiver_id: 'sub-acct2.my-account.testnet',
    actions: [ { Transfer: { deposit: '1000000000000000000000000' } } ],
    signature: 'ed25519:36HQrBYnEPDnRKM92PTpx6N9YXNTjAemxZN3SexsAm5SFDfHjVZAVrcDnFyn6wpvidoG6XoDXd7GCwYwN2L5WcS3',
    hash: 'Fjxpz4mw5v4xoNGXri2Csjbtt62aSD2d7fSWQk3t8YZA'
  },
  transaction_outcome: {
    proof: [
      {
        hash: 'BpSF6qt1uJqZVtAsMK6muX2J6JsGL5wBZ9wq2yQjFv9j',
        direction: 'Right'
      },
      {
        hash: 'ASiRmZaRzZLtiA8heMuEpvQeoAq82Joe4uDdFGQRTrh2',
        direction: 'Left'
      },
      {
        hash: 'fK3hYCNSYAErU8HRtkKGAMnJTT96cn7P4zTup1nuXyg',
        direction: 'Right'
      }
    ],
    block_hash: 'Gbpfz5aNJGugcm5Q1az4Tkz39zwga4SzhJyd58p2HN8y',
    id: 'Fjxpz4mw5v4xoNGXri2Csjbtt62aSD2d7fSWQk3t8YZA',
    outcome: {
      logs: [],
      receipt_ids: [ 'Apu6akhg24vxUogM6QxTg6MdwKBtntYFysXiKbAuyZvz' ],
      gas_burnt: 223182562500,
      tokens_burnt: '22318256250000000000',
      executor_id: 'sub-acct1.my-account.testnet',
      status: {
        SuccessReceiptId: 'Apu6akhg24vxUogM6QxTg6MdwKBtntYFysXiKbAuyZvz'
      }
    }
  },
  receipts_outcome: [
    {
      proof: [
        {
          hash: 'AcTwf8srMXzZoAsgkQZAcLnKHG3QfKcRcnGb4JN3556L',
          direction: 'Left'
        }
      ],
      block_hash: '9VESNEGrogGvAQ444j86NYrvMBiBZiFkFKMM8eyyWRHg',
      id: 'Apu6akhg24vxUogM6QxTg6MdwKBtntYFysXiKbAuyZvz',
      outcome: {
        logs: [],
        receipt_ids: [ 'GCkQmqfpouxQaZE7jLe1nxmRim3H2g55n2TVopTAEiCR' ],
        gas_burnt: 223182562500,
        tokens_burnt: '22318256250000000000',
        executor_id: 'sub-acct2.my-account.testnet',
        status: { SuccessValue: '' }
      }
    },
    {
      proof: [
        {
          hash: '9qJTgfjV9AxMyKaAHmJBN77yxN6ohUvw26ZXRWenMei3',
          direction: 'Left'
        }
      ],
      block_hash: 'AMJ6MqLyRSHzFAGmkkCVkonGfgvsMvFpgUBq9k9CNcAR',
      id: 'GCkQmqfpouxQaZE7jLe1nxmRim3H2g55n2TVopTAEiCR',
      outcome: {
        logs: [],
        receipt_ids: [],
        gas_burnt: 0,
        tokens_burnt: '0',
        executor_id: 'sub-acct1.my-account.testnet',
        status: { SuccessValue: '' }
      }
    }
  ]
}
```

You can also check the state of the 2 subaccounts to confirm the transaction has been done.

```shell
near state sub-acct1.{your account name}.testnet
near state sub-acct2.{your account name}.testnet
```

> Note that, the balance of `sub-acct1` will be slightly less than 29 and the balace of `sub-acct2` is exactly 26. This is because the transaction fee is also needed to be deducted from the sender's account `sub-acct1`.

## Your first contract

### Initialize your Rust project

In the following sections, we use `my-first-contract` as the name of our first smart contract of NEAR.

> If you are NOT familiar to smart contract, you'd better refer to [the official doc](https://docs.near.org/docs/develop/contracts/overview) first for the basic concepts.

Initialize the project by cargo first. In your workspace, simply run:

```shell
cargo init my-first-contract
```

Then edit the `Cargo.toml` file in folder `my-first-contract`. It may seems like:

```toml
[package]
name = "my-first-contract"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
near-sdk = "3.1.0"

[profile.release]
codegen-units = 1
# Tell `rustc` to optimize for small code size.
opt-level = "z"
lto = true
debug = false
panic = "abort"

[workspace]
members = []
```

Note that, we use a specific release version `3.1.0` of `near-sdk` in `dependencies`.

> If you are familiar to rust docs, you can also refer to [the official doc of near-sdk](https://docs.rs/near-sdk/3.1.0/near_sdk/).

Because the smart contract needs to be compiled to WASM binary rather than an executable binary, we specified `lib` settings, which indicated the project as a 'Rust Library'. And we need to change the file name of `main.rs` to `lib.rs`, before we can compile the project.

> For more detail of `crate-type` of Rust, please refer to [the official doc](https://doc.rust-lang.org/reference/linkage.html).

### Add code of your contract

Delete auto-generated content in `lib.rs` and add the following code:

```rust
use near_sdk::{near_bindgen, env};
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct MyFirstContract {
    string_hash_map: HashMap<String, String>
}
```
