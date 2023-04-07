# Clinews

A cli program that shows the top headlines of newsapi for Germany

## Building

### With [Nix](https://nixos.org)

Make sure you're in a directory where you have read permissions
then run the folloing commands

```sh
nix build "github:Kanavaar/rust-clinews"
```

#### Or simply running the program

Requires [Nix](https://nixos.org)

```sh
nix run "github:Kanavaar/rust-clinews"
```

### Without nix

Because there are no dependecies expect cargo you can just run:

```sh
git clone https://github.com/Kanavaar/rust-clinews
cd rust-clinews
cargo run
```


## License

Apache-2.0

More information at <https://github.com/Kanavaar/rust-clinews/blob/main/LICENSE>
