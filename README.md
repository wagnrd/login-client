# Login Client

## Setup

### Linux

Fedora:

```shell
sudo dnf install gtk4-devel gcc libadwaita-devel openssl-devel
```

Ubuntu:

```shell
sudo apt install libgtk-4-dev build-essential libadwaita-1-dev libssl-dev
```

### macOs

```shell
brew install gtk4 libadwaita
```

### Windows

- [GTK 4](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)
- [Libadwaita](https://gtk-rs.org/gtk4-rs/stable/latest/book/libadwaita.html)

## Build and run

For development first you have to build the test app:

```shell
cargo build --workspace
```

And run it with:

```shell
cargo run
```