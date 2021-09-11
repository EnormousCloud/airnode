[[ "$ENORMOUS_RPC_API_TOKEN" == "" ]] && {
    echo "ENORMOUS_RPC_API_TOKEN is not provided"
    exit 1
}

[[ "$1" == "xdai" ]] && {
    export RPC_ENDPOINT=http://xdai.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
    export RPC_BATCH_SIZE=10000
    export RPC_MIN_BLOCK=13796900
    export ADDR_CONTRACT=0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE
    cargo run --release
}

[[ "$1" == "rinkeby" ]] && {
    export RPC_ENDPOINT=http://rinkeby.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=7812260
    export ADDR_CONTRACT=0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
    cargo run --release
}

[[ "$1" == "rinkeby2" ]] && {
    # something from Hackathon
    export RPC_ENDPOINT=http://rinkeby.enormous.cloud/$ENORMOUS_RPC_API_TOKEN
    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=8494000
    export ADDR_CONTRACT=0xb8e96200a1290907436d928bcc3c7ff18e7f4ae6
    cargo run --release
}

[[ "$1" == "" ]] && {
    echo "missing parameter: xdai,rinkeby,rinkeby2"
    exit 1
}


