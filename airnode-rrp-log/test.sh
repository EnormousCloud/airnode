[[ "$1" == "xdai" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t xdai | head -n1)
    [[ "RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }
    export RPC_BATCH_SIZE=100000
    export RPC_MIN_BLOCK=13796900
    export ADDR_CONTRACT=0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE
    cargo run --release -- $@
    exit
}

[[ "$1" == "rinkeby" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=7812260
    export ADDR_CONTRACT=0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
    cargo run --release -- $@
    exit
}

[[ "$1" == "rinkeby2" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=8494000
    export ADDR_CONTRACT=0xb8e96200a1290907436d928bcc3c7ff18e7f4ae6
    cargo run --release -- $@
    exit
}

[[ "$1" == "rinkeby0.3" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    export RPC_BATCH_SIZE=1000000
    export RPC_MIN_BLOCK=9780500
    export ADDR_CONTRACT=0xC11593B87f258672b8eB02d9A723a429b15E9E03
    cargo run --release -- $@
    exit
}

[[ "$1" == "" ]] && {
    echo "missing parameter: xdai,rinkeby,rinkeby2,rinkeby0.3"
    exit 1
}
