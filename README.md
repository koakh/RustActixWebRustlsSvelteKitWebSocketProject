# README

## RustActixWebRustlsSvelteKitWebSocketProject

a rust poc with actixweb and rustls backend + and sveltekit websocket client frontend

## Links

- [HTTPS Server (using Rustls)](https://github.com/actix/examples/tree/master/https-tls/rustls)

from `actix/examples/tree/master/https-tls/rustls`

## Generate your own cert/private key files

If you want to generate your own cert/private key file, then run:

```shell
$ mkdir /tmp/certs
$ cd /tmp/certs

$ mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost

Note: the local CA is not installed in the system trust store.
Note: the local CA is not installed in the Firefox and/or Chrome/Chromium trust store.

Run "mkcert -install" for certificates to be trusted automatically

Created a new certificate valid for the following names
 - "127.0.0.1"
 - "localhost"

The certificate is at "cert.pem" and the key at "key.pem"

It will expire on 2 March 2026 🗓
```

## Run Project

### Server

```shell
$ cd server
$ cargo run
[2023-11-30T22:18:36Z INFO  actixweb_rustls_server_example] starting HTTPS server at https://localhost:8443
```

### Client

```shell
$ cd client
$ pnpm i
$ pnpm dev
  ➜  Local:   http://localhost:5173/
```

