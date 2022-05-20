set -xe
export STATIC_DIR=$(pwd)/dist

[[ "$1" == "build" ]] && {
    shift
    trunk build
    cd dist; export FN=$(find . -name 'index-*.wasm'); wasm-opt -Os $FN --output out.wasm && mv out.wasm $FN
    cd -
    docker build -t api3-rrp-explorer .
}

[[ "$1" == "build-mainnet" ]] && {
    shift
    trunk build --public-url=/dao/api3/rrp-explorer/ 
    cd dist; export FN=$(find . -name 'index-*.wasm'); wasm-opt -Os $FN --output out.wasm && mv out.wasm $FN
    cd -
    docker build -t api3-rrp-explorer .
}

[[ "$1" == "publish-mainnet" ]] && {
    export SSH_HOST="root@enormous.cloud"
    docker save api3-rrp-explorer | bzip2 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null $SSH_HOST 'bunzip2 | docker load'
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
       $SSH_HOST 'cd /opt/api3-rrp-explorer; docker rm -f api3-rrp-explorer; docker-compose up -d'
}
