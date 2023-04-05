# Building a kernel only in rust

Got the code and the idea from: https://os.phil-opp.com/ 

there is all the details of why things are things and some other magic stuff that looks like magic if you dont undestand it well


## Build the binaries

```bash
Verbose:

Other persons target build
$ cargo build --target thumbv7em-none-eabihf

or:

Our way to do it
$ cargo build --target x84_64-bootloader.json
```

Because we defined in our .cargo/config.toml file to target a file in the build we can just use

cargo build

and will do the job


We added bootloader to our dependecies and we added a rust binary

cargo install bootimage 

that will make our bootloader binary.

after that run cargo bootimage

but for that to work you will need to have llvm-tools-preview and to get that do:

rustup component add llvm-tools-preview


for it to work you will need the qemu for x84_64 processors
