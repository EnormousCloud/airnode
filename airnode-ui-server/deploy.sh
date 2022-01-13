#!/usr/bin/env bash

set -ex

[[ "$1" == "build" ]] && {
    shift
    docker build -t airnode-ui-server .. -f ./Dockerfile 
}

[[ "$1" == "run" ]] && {
    shift
    docker rm -f airnode-ui-server || true
    docker run -p 8000:8000 -d \
        -v $(pwd)/_data:/app/_data \
        -v $(pwd)/_www:/app/_www  \
        --name airnode-ui-server \
        airnode-ui-server
    docker logs -f airnode-ui-server
}

[[ "$1" == "upload-api" ]] && {
    shift
    docker save airnode-ui-server | bzip2 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
        root@enormous.cloud 'bunzip2 | docker load'
}

[[ "$1" == "upload-db" ]] && {
    shift
    scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
       -r ./_data/ root@enormous.cloud:/opt/api3-airnodes/
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
       root@enormous.cloud chmod -R 0777 /opt/api3-airnodes/_data
}

[[ "$1" == "upload-www" ]] && {
    shift
    scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
       -r ./_www/ root@enormous.cloud:/opt/api3-airnodes/
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
       root@enormous.cloud chmod -R 0777 /opt/api3-airnodes/_www
}
