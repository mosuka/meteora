# Meteora

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Meteora is a distributed key-value store written in [Rust](https://www.rust-lang.org/) built on top of [RocksDB](https://rocksdb.org/) and implemented by [The Raft Consensus Algorithm](https://raft.github.io/) and [The gRPC](https://grpc.io/).  
Achieves consensus across all the nodes, ensures every change made to the system is made to a quorum of nodes.  
Meteora makes easy for programmers to develop an applications with advanced features and high availability.


```shell
$ ./bin/meteora start -i 1 -a 0.0.0.0 -r 7001 -k 5001 -d /tmp/meteora/1/data
$ ./bin/meteora start -i 2 -a 0.0.0.0 -r 7002 -k 5002 -d /tmp/meteora/2/data -p 0.0.0.0:7001
$ ./bin/meteora start -i 3 -a 0.0.0.0 -r 7003 -k 5003 -d /tmp/meteora/3/data -p 0.0.0.0:7001
```

```shell
$ grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/raft.proto -plaintext 0.0.0.0:7001 meteora.raft.RaftService/Status | jq .
```


```shell
$ ./bin/meteora put -a 0.0.0.0:7001 key1 "Meteora is a distributed key-value store."
$ ./bin/meteora get -a 0.0.0.0:5001 key1
$ ./bin/meteora get -a 0.0.0.0:5002 key1
$ ./bin/meteora get -a 0.0.0.0:5003 key1
```

```shell
$ export KEY="key1" && export VALUE="value1" && grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/kv.proto -d "{ \"key\": \"$(echo -n $KEY | base64 -)\", \"value\": \"$(echo -n $VALUE | base64 -)\"}" -plaintext 0.0.0.0:5001 meteora.kv.KvService/Put | jq .
$ export KEY="key1" && grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/kv.proto -d "{ \"key\": \"$(echo -n $KEY | base64 -)\" }" -plaintext 0.0.0.0:5001 meteora.kv.KvService/Get | jq '. | { "value" : .value | @base64d }'
$ export KEY="key1" && grpcurl -import-path meteora-proto/proto -proto meteora-proto/proto/kv.proto -d "{ \"key\": \"$(echo -n $KEY | base64 -)\" }" -plaintext 0.0.0.0:5001 meteora.kv.KvService/Delete | jq .
```
