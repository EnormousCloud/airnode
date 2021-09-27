# airnode-rrp-log

### Installation

```cargo install airnode-rrp-log```

### Usage
```
USAGE:
    airnode-rrp-log [OPTIONS] --address-contract <address-contract>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --address-contract <address-contract>    API3 secondary voting agent address [env: ADDR_CONTRACT=]
        --max-block <max-block>                  Max block to stop contract events listening [env: RPC_MAX_BLOCK=]
        --min-block <min-block>
            Number of the first block to start watching [env: RPC_MIN_BLOCK=]  [default: 1]

        --rpc-batch-size <rpc-batch-size>
            Ethereum JSON+RPC batch size for reading. Light clients will require smaller sizes [env: RPC_BATCH_SIZE=]
            [default: 1000]
        --rpc-endpoint <rpc-endpoint>
            Ethereum JSON+RPC HTTP address [env: RPC_ENDPOINT=]  [default: http://localhost:8545/]
```

### Testing with XDAI

```
export RPC_ENDPOINT=http://xdai.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
export RPC_BATCH_SIZE=10000
export RPC_MIN_BLOCK=13796900
export ADDR_CONTRACT=0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE
airnode-rrp-log
```

### Testing with Rinkeby

```
export RPC_ENDPOINT=http://rinkeby.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
export RPC_BATCH_SIZE=50000
export RPC_MIN_BLOCK=7812260
export ADDR_CONTRACT=0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
airnode-rrp-log
```

### See Also

UI with the same functionality: 
- Demo: https://enormous.cloud/dao/api3/rrp-explorer
- Source: https://github.com/EnormousCloud/airnode/tree/main/airnode-rrp-explorer

### License
MIT