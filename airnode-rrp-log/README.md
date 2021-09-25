# airnode-rrp-log

```
USAGE:
    airnode-rrp-log \
        --address-contract <address-contract> \
        --min-block <min-block> \
        --rpc-batch-size <rpc-batch-size> \
        --rpc-endpoint <rpc-endpoint>
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