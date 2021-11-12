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
        --batch-size 50000
    # display all RRP operations
    cargo run --release -- op list \
        --chain-id 4 \
        --contract-address 0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
    exit
}

cargo run --release -- server
