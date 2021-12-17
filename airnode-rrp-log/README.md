# airnode-rrp-log

### Installation

```cargo install airnode-rrp-log```

### Usage
```
USAGE:
    airnode-rrp-log [FLAGS] [OPTIONS] --address-contract <address-contract>

FLAGS:
    -h, --help            Prints help information
        --pretty-print    Pretty print JSON responses
    -V, --version         Prints version information

OPTIONS:
        --address-contract <address-contract>        API3 secondary voting agent address [env: ADDR_CONTRACT=]
        --by-address <by-address>                    [env: BY_ADDRESS=]
        --by-airnode <by-airnode>                    [env: BY_AIRNODE=]
        --by-endpoint-id <by-endpoint-id>            [env: BY_ENDPOINT_ID=]
        --by-provider-id <by-provider-id>            [env: BY_PROVIDER_ID=]
        --by-request-id <by-request-id>              [env: BY_REQUEST_ID=]
        --by-requester-index <by-requester-index>    [env: BY_ENDPOINTER_INDEX=]
        --by-template-id <by-template-id>            [env: BY_TEMPLATE_ID=]
    -f, --format <format>                            format of output: "jsonl" or "json" [default: jsonl]
        --max-block <max-block>                      Max block to stop contract events listening [env: RPC_MAX_BLOCK=]
        --min-block <min-block>                      Number of the first block to start watching [env: RPC_MIN_BLOCK=9780500]  [default: 1]
        --rpc-batch-size <rpc-batch-size>            Ethereum JSON+RPC batch size for reading. Light clients will require smaller size  [env:RPC_BATCH_SIZE=1000000]  [default: 1000]
        --rpc-endpoint <rpc-endpoint>                Ethereum JSON+RPC HTTP address [env:RPC_ENDPOINT=https://rinkeby.infura.io/v3/.....]  [default:http://localhost:8545/]
```

### Testing with XDAI

```
export RPC_ENDPOINT=$(chainstate --endpoints -t xdai | head -n1)
export RPC_BATCH_SIZE=100000
export RPC_MIN_BLOCK=13796900
export ADDR_CONTRACT=0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AEairnode-rrp-log
airnode-rrp-log
```

### Testing with Rinkeby

```
export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
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