# How to install Rust Lang on Ubuntu

## Installing installation flow dependencies

If you don't have curl and the C lang linker installed run the commands below to install both.

```
sudo apt update
sudo apt install -y curl build-essential
```

## Installing rust lang

Run the command below to install the rust lang.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You will see something like the text box below, select the default installation mode by entering 1 and hitting enter.

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>
```

Then you will see the message (**Rust is installed now. Great!**) if everything goes well.
