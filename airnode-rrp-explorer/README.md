# airnode-rrp-explorer

RRP Explorer is a client-side WASM script to explore logs of airnode on any EVM-compatible network

- Demo: https://enormous.cloud/dao/api3/rrp-explorer
- CLI with the same functionality: https://github.com/EnormousCloud/airnode/tree/main/airnode-rrp-log

### Compilation

This tool uses https://trunkrs.dev/ and generates HTML with WASM executable.

To compile use `trunk build` and serve static files from `dist/` folder

### Testing on XDAI

```
RPC_ENDPOINT=http://xdai.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
RPC_BATCH_SIZE=10000
RPC_MIN_BLOCK=13796900
ADDR_CONTRACT=0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE
```

### Testing on Rinkeby

```
RPC_ENDPOINT=http://rinkeby.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
RPC_BATCH_SIZE=50000
RPC_MIN_BLOCK=7812260
ADDR_CONTRACT=0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
```

### Coming features
- [ ]  highlight decoding errors with another color
- [ ]  slowdown/hanging during the load: get rid of accumulative decoding on each render
- [ ]  keep the submission form input state in the local storage
- [ ]  no good indication of batches that are processing
- [ ]  filtration: by provider ID
- [ ]  filtration: by endpoint ID
- [ ]  filtration: by request ID
- [ ]  filtration: by requester index
- [ ]  export results as CSV and/or plain text

### License
MIT