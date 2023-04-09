# Clinews

A cli program that shows the top headlines of newsapi for Germany

![Logo of clinews](logo.png)

## Building

### With [Nix](https://nixos.org)

Make sure you're in a directory where you have read permissions
then run the folloing commands

```sh
nix build "git+https://codeberg.org:tilmanmixyz/clinews"
```

#### Or simply running the program

Requires [Nix](https://nixos.org)

```sh
nix run "git+https://codeberg.org:tilmanmixyz/clinews"
```

### Without nix

Because there are no dependecies expect cargo you can just run:

```sh
git clone https://codeberg.org/tilmanmixyz/clinews
cd clinews
cargo run
```


## License

Apache-2.0

More information at <https://github.com/tilmanmixyz/clinews/blob/main/LICENSE>
