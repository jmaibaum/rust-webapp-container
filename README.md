# Preparation

## Install

### Containerization

podman buildah fuse-overlayfs


### WASM frontend stuff

cargo install -f cargo-web


## Configure

### User namespaces (is this needed?)

    $ sudo sysctl kernel.unpriviledged_userns_clone=1

Make setting permanent using `systctl.d` if needed.


### /etc/subuid

    <user-name>:100000:65536


### /etc/subgid

    <user-name>:100000:65536


# rust-musl-builder

Set up alias:

    $ alias rust-musl-builder='podman run --rm -it --userns keep-id -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'


# Build server

    $ rust-musl-builder cargo build -p webapp --release


# Build frontend

    $ cargo web deploy -p frontend


# Build container

    $ buildah build-using-dockerfile -t <image-name> .


# Start container

    $ podman run -dt --rm --net host -p 8000:8000/tcp --name <container-name> <image-name>


# List running containers

    $ podman ps -a


# Stop container

    $ podman stop --latest

or

    $ podman stop <name>


# Remove container

If creating a container doesn't work, because podman complains that there is
already a container using that name, and then trying to remove said container
using:

    $ podman rm <container-name>

doesn't work, because podman cannot find a container with that name, try:

    $ podman rm --storage <container-name>
