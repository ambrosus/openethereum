## Usage

```docker build -f docker/apline/Dockerfile --tag ethcore/openethereum:branch_or_tag_name .```

All default ports you might use will be exposed:
```
#      secret
#      store     ui   rpc  ws   listener  discovery
#      ↓         ↓    ↓    ↓    ↓    ↓         ↓
EXPOSE 8082 8083 8180 8545 8546 30303/tcp 30303/udp
```
