# static-website-host
A generalised Rust webserver for hosting static website files.


## Installation
There are two ways of building the server binary: either locally or as a Docker container.

### Native build
To build the binary locally, first install [Rust](https://rust-lang.org). We recommend using [rustup](https://rustup.rs):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then you can build the project by running:
```sh
cargo build --release
```

The resulting binary is found under `target/release/static-website-host`.


### Docker build
To build the container in Docker, run:
```sh
docker build -t static-website-host -f Dockerfile --target release .
```

> NOTE: If you're using Docker's buildx plugin as standard, don't forget to specify `--load`:
> ```sh
> docker build --load -t static-website-host:latest -f Dockerfile --target release .
> ```

This creates a `static-website-host` image in your local Docker daemon.


## Usage
To use the server, create a `www` directory and put your website files in it.

Then, launch the server. If you installed it [natively](#native-build), run:
```sh
./target/release/static-website-host
```

If you installed it with [Docker](#docker-build), run:
```sh
docker run -d --name static-website-host -v "$(pwd)/www:/www" -v "$(pwd)/config.yml:/config.yml"  -p 42080:42080 static-website-host --address 0.0.0.0:42080
```

In both cases, you can now access your site under `http://localhost:42080`.

You can also launch your server using Docker Compose if you've built with Docker:
```sh
docker compose up -d
```

Note that in that case, the server is launched under port `80` instead of `42080`.

### Config
To configure the server, look at `config.yml`:
```yaml
# The location of the website files to host.
site: './www'
# The location of the page that is shown when a user triggers a 404 (NOT FOUND).
not_found_file: './www/not_found.html'
```
Either is generated if it doesn't exist yet.


## Contributions
Contributions to this project are welcome! Create an [issue](Lut99/static-website-host/issues) if you have a question, idea or encountered a bug; or go ahead and create a [pull request](Lut99/static-website-host/pulls) if you already did the change yourself.


## License
This project is licensed under GPLv3. See [LICENSE](./LICENSE) for more details.
