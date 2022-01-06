# airnode-ui-server

Server API to download and watch for the state of Airnodes RRP contracts on EVM chains

```
OPTIONS:
    -d, --data-dir <data-dir>    Directory for persistent data storage [env: DATA_DIR=]  [default: ./_data]

SUBCOMMANDS:
    config    commands to manage Airnode RRP contracts
    op        commands to manage operations
    server    start HTTP server
    state     commands to read and return state of the airnode
```

### Server Command
```
FLAGS:
        --no-sync    Skip syncing
OPTIONS:
        --listen <listen>    Net listening address of HTTP server in case of "server" command [env: LISTEN=]  [default: 0.0.0.0:8000]
```

### Config Command
```
SUBCOMMANDS:
    add       Add airnode RRP contract to the configuration
    delete    Delete airnode RRP configuration and erase all its history
    get       Get airnode RRP contract configuration
    list      Get the list of RRP contracts
    update    Update airnode RRP contract configuration
```

### Op Command
```
SUBCOMMANDS:
    list        Get the list of operations
    truncate    Reset operations log
```

### State COmmand
```
SUBCOMMANDS:
    get     Get the state of the single node
    list    List states for all nodes
```


### License
MIT