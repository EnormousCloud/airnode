[[ "$1" == "ankr" ]] && {
    shift
    export RPC_ENDPOINT=https://rpc.ankr.com/eth
    export ADDR_CONTRACT=0xa0AD79D995DdeeB18a14eAef56A549A04e3Aa1Bd
   export RPC_BATCH_SIZE=3000
    export RPC_MIN_BLOCK=14622752
    export RPC_MAX_BLOCK=14698580
    cargo run --release -- $@
    exit
}

[[ "$1" == "cloudflare" ]] && {
    shift
    export RPC_ENDPOINT=https://cloudflare-eth.com/
    export ADDR_CONTRACT=0xa0AD79D995DdeeB18a14eAef56A549A04e3Aa1Bd
    cargo run --release -- $@
    exit
}

[[ "$1" == "mainnet" ]] && {
    shift
set -x
    export RPC_ENDPOINT=$(chainstate --endpoints -t mainnet,alchemy | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for mainnet network. "; exit 1; }
    export RPC_BATCH_SIZE=25000
    export RPC_MIN_BLOCK=14622752
    export RPC_MAX_BLOCK=14698580
    export ADDR_CONTRACT=0xa0AD79D995DdeeB18a14eAef56A549A04e3Aa1Bd
    cargo run --release -- $@
    exit
}

[[ "$1" == "xdai" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t xdai | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }
    export RPC_BATCH_SIZE=100000
    export RPC_MIN_BLOCK=13796900
    export ADDR_CONTRACT=0x32D228B5d44Fd18FefBfd68BfE5A5F3f75C873AE
    cargo run --release -- $@
    exit
}

[[ "$1" == "rinkeby-prealpha" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=7812260
    export ADDR_CONTRACT=0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec
    cargo run --release -- $@
    exit
}

[[ "$1" == "rinkeby-prealpha2" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=8494000
    export ADDR_CONTRACT=0xb8e96200a1290907436d928bcc3c7ff18e7f4ae6
    cargo run --release -- $@
    exit
}

[[ "$1" == "ropsten-0.3" ]] && {
    shift
    set -ex
    export RPC_ENDPOINT=$(chainstate --endpoints -t ropsten | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for ropsten network. "; exit 1; }

    export RPC_BATCH_SIZE=1000000
    export RPC_MIN_BLOCK=11329160
    export ADDR_CONTRACT=0x3B35250Ca54C1Fb8c83D48F21231ef6e4fb9f79D
    cargo run --release -- $@
    exit
}

[[ "$1" == "rinkeby-0.3" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rinkeby,infura | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for rinkeby network. "; exit 1; }

    export RPC_BATCH_SIZE=1000000
    export RPC_MIN_BLOCK=9780500
    export ADDR_CONTRACT=0xC11593B87f258672b8eB02d9A723a429b15E9E03
    cargo run --release -- $@
    exit
}

[[ "$1" == "rsk-testnet-prealpha" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rsk,testnet | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for RSK-testnet network. "; exit 1; }

    export RPC_BATCH_SIZE=50000
    export RPC_MIN_BLOCK=1817900
    export ADDR_CONTRACT=0x1190a5e1f2afe4c8128fd820a7ac85a95a9e6e3e
    cargo run --release -- $@
    exit
}

[[ "$1" == "rsk-testnet-0.3" ]] && {
    shift
    export RPC_ENDPOINT=$(chainstate --endpoints -t rsk,testnet | head -n1)
    [[ "$RPC_ENDPOINT" == "" ]] && { echo "missing RPC endpoint for RSK-testnet network. "; exit 1; }

    export RPC_BATCH_SIZE=100000
    export RPC_MIN_BLOCK=2421800
    export ADDR_CONTRACT=0x93ef1c3c090e2748b88a5cb91d350a53b8f9cbe8
    cargo run --release -- $@
    exit
}

[[ "$1" == "" ]] && {
    echo "missing parameter: xdai,rinkeby-prealpha,rinkeby-prealpha2,ropsten-0.3,rinkeby-0.3,rsk-testnet-prealpha,rsk-testnet-0.3"
    exit 1
}
