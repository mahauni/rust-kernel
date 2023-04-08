# Building a kernel only in rust

Got the code and the idea from: https://os.phil-opp.com/ 

there is all the details of why things are things and some other magic stuff that looks like magic if you dont undestand it well


## Building the binaries:

First you will need to clone this repo and enter the folder:

```bash
$ git clone https://github.com/mahauni/rust-kernel

$ cd ./rust-kernel
```

Then you will need to have rust nightly, and other dependencies. You can install rust in the https://www.rust-lang.org/tools/install.
The rust nightly, after installing the rust execute this command to install the nightly toolchain and set the nightly to use in the current directory

```bash
$ rustup install nightly

$ rustup override set nightly
```

### After setting up you should be able to build the binaries of the kernel. 

You can build the binaries with our target builds or with others.

If you want to install with the help of other, you can do:

```bash
$ rustup target add thumbv7em-none-eabihf

$ cargo build --target thumbv7em-none-eabihf
```

### Or you can build with ours:

Our bootloader will need some adicional steps to build it.

First you will need to install our boot image.

```bash
$ cargo install bootimage

$ rustup component add llvm-tools-preview

$ cargo bootimage
```

All that is to make the binaries of our boot loader. 

After installed you will have the binaries of our boot loader, and then:

```bash
$ cargo build --target x84_64-bootloader.json
```

But because we defined in our .cargo/config.toml file to target the specific file with the specs of our build, you can just.
```bash
$ cargo build
```
And it will do the job.

After the build the binaries will be in the ./target/x86_64-bootloader/debug or in the .target/thumbv7em-none-eabihf/debug depending the target you choose to build with


### Running the kernel

You will be able to run this kernel in any machine you'd like. But be careful when switching kernels, because if you choose the wrong options when installing a new kernel, it is possible to you remove you current kernel and lose everything. So you can install a app that will do the hassle for you.


Our choise if qemu. You will need to install qemu https://www.qemu.org/ and make it to run in x84_64 processors.

After installed you should be able to run:

```bash
$ cargo run
```

and it will pop a screen with our kernel running in our computer without much hassle. And then you should be play with it.

The cargo run work in this way because we defined in our .cargo/config.toml to run the custom command when running the cargo. Because the command to initialize the kernel is quite big, so we make it easier for us.


### Tests:

You can test the functionalities of the kernel and make sure nothing is breaking. To run the test:

```bash
$ cargo test
```

And then it should be able to run all the test in our /tests folder.


## For now the tutorial of the person is now complete

But its not nowhere done, there still some thing to implement. Some thing there is on https://os.phil-opp.com/async-await/ and there you should have some implementations that are challenging.

Re read more about the async-await, because i didnt understand all of it, too much information and is too strange now. But is a good thing to know, so read later and try to understand it.
