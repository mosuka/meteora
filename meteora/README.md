# Meteora

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Meteora is a distributed key-value store written in [Rust](https://www.rust-lang.org/) built on top of [RocksDB](https://rocksdb.org/) and implemented by [The Raft Consensus Algorithm](https://raft.github.io/) and [The gRPC](https://grpc.io/).  
Achieves consensus across all the nodes, ensures every change made to the system is made to a quorum of nodes.  
Meteora makes easy for programmers to develop an applications with advanced features and high availability.

## Start in standalone mode (Single node cluster)

Running node in standalone mode is easy. You can start server with the following command:

```shell
meteora start
```

Then you can see the cluster state with the following command:

```shell
grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/raft.proto -plaintext 0.0.0.0:7000 meteora.raft.RaftService/Status | jq .
```

You'll see the result in JSON format. The result of the above command is:

```json
{
  "state": "OK",
  "addressMap": {
    "1": {
      "kvAddress": "0.0.0.0:5000",
      "raftAddress": "0.0.0.0:7000"
    }
  },
  "leaderId": "1"
}
```

### Put data

Meteora can communicate with gRPC, and you can use the `grpcurl` command to put your data. The following command is an example of putting a value `value1` for a key `key1`.

```shell
export KEY="key1" && export VALUE="value1" && grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/kv.proto -d "{ \"key\": \"$(echo -n $KEY | base64 -)\", \"value\": \"$(echo -n $VALUE | base64 -)\"}" -plaintext 0.0.0.0:5000 meteora.kv.KvService/Put | jq .
```

If the putting data is successful, you will see the result in JSON format as shown below:

```json
{
  "state": "OK",
  "addressMap": {
    "1": {
      "kvAddress": "0.0.0.0:5000",
      "raftAddress": "0.0.0.0:7000"
    }
  },
  "leaderId": "1"
}
```

### Get data

The following command is an example of getting data with key `key1`.

```shell
export KEY="key1" && grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/kv.proto -d "{ \"key\": \"$(echo -n $KEY | base64 -)\" }" -plaintext 0.0.0.0:5000 meteora.kv.KvService/Get | jq '. | { "value" : .value | @base64d, "state" : .state, "addressMap" : .addressMap, "leaderId" : .leaderId }'
```

If the getting data is successful, you will see the result in JSON format as shown below:

```json
{
  "value": "value1",
  "state": "OK",
  "addressMap": {
    "1": {
      "kvAddress": "0.0.0.0:5000",
      "raftAddress": "0.0.0.0:7000"
    }
  },
  "leaderId": "1"
}
```

### Deleting data

The following command is an example of getting data with key `key1`.

```shell
export KEY="key1" && grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/kv.proto -d "{ \"key\": \"$(echo -n $KEY | base64 -)\" }" -plaintext 0.0.0.0:5000 meteora.kv.KvService/Delete | jq .
```

If the deleting data is successful, you will see the result in JSON format as shown below:

```json
{
  "state": "OK",
  "addressMap": {
    "1": {
      "kvAddress": "0.0.0.0:5000",
      "raftAddress": "0.0.0.0:7000"
    }
  },
  "leaderId": "1"
}
```

## Start in cluster mode (3 nodes cluster)

Running in standalone is not fault tolerant. If you need to improve fault tolerance, you need to start Meteora in cluster mode. Meteora already supports the cluster mode, and easily bringing up a cluster by the following command:

```shell
meteora start -i 1 -a 0.0.0.0 -r 7001 -k 5001 -d /tmp/meteora/1/data -l
```

```shell
meteora start -i 2 -a 0.0.0.0 -r 7002 -k 5002 -d /tmp/meteora/2/data -p 0.0.0.0:7001 -l
```

```shell
meteora start -i 3 -a 0.0.0.0 -r 7003 -k 5003 -d /tmp/meteora/3/data -p 0.0.0.0:7001 -l
```

Then you can see the cluster state with the following command:

```shell
grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/raft.proto -plaintext 0.0.0.0:7001 meteora.raft.RaftService/Status | jq .
```

You'll see the result in JSON format. The result of the above command is:

```json
{
  "state": "OK",
  "addressMap": {
    "1": {
      "kvAddress": "0.0.0.0:5001",
      "raftAddress": "0.0.0.0:7001"
    },
    "2": {
      "kvAddress": "0.0.0.0:5002",
      "raftAddress": "0.0.0.0:7002"
    },
    "3": {
      "kvAddress": "0.0.0.0:5003",
      "raftAddress": "0.0.0.0:7003"
    }
  },
  "leaderId": "1"
}
```

The above commands run nodes on the same host, so each node must listen on a different port. This would not be necessary if each node runs on a different host. Recommend 3 or more odd number of nodes in the cluster to avoid split-brain.
When deploying to a single host, if that host goes down due to hardware failure, all of the servers in the cluster will be stopped, so recommend deploying to a different host.


## Using Docker container

See the available Docker container image version at the following URL:
- [https://hub.docker.com/r/meteorakvs/meteora/tags/](https://hub.docker.com/r/meteorakvs/mteora/tags/)

### Running on Docker

You can run the Docker container image with the following command:

```shell
docker run --rm --name meteora \
       -p 5000:5000 -p 7000:7000 \
       meteorakvs/meteora:latest
```
