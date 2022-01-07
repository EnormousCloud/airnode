#!/usr/bin/env bash

set -e

export LOG_LEVEL=airnode_ui_server=info,info

[[ "$1" == "rinkeby-prealpha" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    # adding RRP contract to the configuration
    cargo run --release -- config add \
        --contract-address 0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec \
        --min-block 7812260 \
        --batch-size 100000
    # display all RRP operations
    cargo run --release -- op list \
        --chain-id 4 \
        --contract-address 0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
    exit
}

[[ "$1" == "rinkeby-prealpha2" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    # adding RRP contract to the configuration
    cargo run --release -- config add \
        --contract-address 0xb8e96200a1290907436d928bcc3c7ff18e7f4ae6 \
        --min-block 8494000 \
        --batch-size 100000
    # display all RRP operations
    cargo run --release -- op list \
        --chain-id 4 \
        --contract-address 0xb8e96200a1290907436d928bcc3c7ff18e7f4ae6
    exit
}

[[ "$1" == "xdai-prealpha" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t xdai | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for xdai network. "; exit 1; }

    # adding RRP contract to the configuration
    cargo run --release -- config add \
        --contract-address 0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE \
        --min-block 13796900 \
        --batch-size 100000
    # display all RRP operations
    cargo run --release -- op list \
        --chain-id 100 \
        --contract-address 0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE
    exit
}

[[ "$1" == "rsk-testnet-prealpha" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rsk,testnet,localhost | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for RSK-testnet network. "; exit 1; }

    # adding RRP contract to the configuration
    cargo run --release -- config add \
        --contract-address 0x1190a5e1f2afe4c8128fd820a7ac85a95a9e6e3e \
        --min-block 1817900 \
        --batch-size 100000
    # display all RRP operations
    cargo run --release -- op list \
        --chain-id 31 \
        --contract-address 0x1190a5e1f2afe4c8128fd820a7ac85a95a9e6e3e 
    exit
}

[[ "$1" == "rinkeby-0.3" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for Rinkeby network. "; exit 1; }

    # adding RRP contract to the configuration
    cargo run --release -- config add \
        --contract-address 0xC11593B87f258672b8eB02d9A723a429b15E9E03 \
        --min-block 9780500 \
        --batch-size 500000
    # display all RRP operations
    cargo run --release -- op list \
        --chain-id 4 \
        --contract-address 0xC11593B87f258672b8eB02d9A723a429b15E9E03 
    exit
}

[[ "$1" == "ropsten-0.3" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t ropsten | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for Ropsten network. "; exit 1; }

    # adding RRP contract to the configuration
    cargo run --release -- config add \
        --contract-address 0x3B35250Ca54C1Fb8c83D48F21231ef6e4fb9f79D \
        --min-block 11329160 \
        --batch-size 100000
    # display all RRP operations
    export LOG_LEVEL=airnode_ui_server::storage_ops=debug,info
    cargo run --release -- op list \
        --chain-id 3 \
        --contract-address 0x3B35250Ca54C1Fb8c83D48F21231ef6e4fb9f79D 
    exit
}

cargo run --release -- server
