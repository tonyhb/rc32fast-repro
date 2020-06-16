# crc32fast wasm repro

Mininal repro for inspecfing memcpy when unzipping via wasm.

To run:

```
make build-web make run-web
```

And visit 127.0.0.1:8888.  Drop a file or choose a file.  The smallest file is recommended!  Anything over 1mb spins.
