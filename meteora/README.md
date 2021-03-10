# Meteora

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Meteora is a distributed key-value store written in [Rust](https://www.rust-lang.org/) built on top of [RocksDB](https://rocksdb.org/) and implemented by [The Raft Consensus Algorithm](https://raft.github.io/) and [The gRPC](https://grpc.io/).  
Achieves consensus across all the nodes, ensures every change made to the system is made to a quorum of nodes.  
Meteora makes easy for programmers to develop an applications with advanced features and high availability.


```shell
$ ./bin/meteora start -H 0.0.0.0 -r 7001 -k 5001 -d /tmp/meteora/1/data 1
$ ./bin/meteora start -H 0.0.0.0 -r 7002 -k 5002 -p 0.0.0.0:7001 -d /tmp/meteora/2/data 2
$ ./bin/meteora start -H 0.0.0.0 -r 7003 -k 5003 -p 0.0.0.0:7001 -d /tmp/meteora/3/data 3
```