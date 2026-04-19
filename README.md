# Playground for high performance network communication

## Goal

 - [x] Evaluate gRPC including stream
 - [ ] Build benchmark for bandwidth and latency use Perf, eBPF
 - [ ] Benchmark XDP approach
 - [ ] Benchmark eBPF approach
 - [ ] Benchmark DPDK approach
 - [ ] Benchmark RDMA approach
 - [ ] Benchmark io_uring approach

## What to benchmark

The `gsrv` provide a bidirectional stream to `gcli` to reply massive
mount(10,000,000) of hello(string).

Both `gsrv` and `gcli` are communicated through `http://[::1]:50051` address.
TODO(Move to different VM for real NIC communication).

Generate report on:
 - [x] Total time for CLI to send enough hello and got enough reply
 - [ ] Top 10 of most time-consuming kernel functions
