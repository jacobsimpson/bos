# bos => Basic Operating System

Follows along with https://os.phil-opp.com/.

## Details

x86_64-bos.json - This file contains the configuration for a compilation target platform.

## Development

To build a bootable image:

```
cargo bootimage
ls target/x86_64-bos/debug
```

To boot the image in a VM:

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-bos/debug/bootimage-bos.bin
```

Or, there is a specialized `run` target to handle that:

```
cargo run
```
