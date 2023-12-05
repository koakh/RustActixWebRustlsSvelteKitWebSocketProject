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
```

## Run Project

### Server

```shell
$ cd server
$ cargo run
[2023-11-30T22:18:36Z INFO  actixweb_rustls_server_example] starting HTTPS server at https://localhost:8443
```

1. test https endpoints 
   1. go to <https://localhost:8443> or <https://localhost:8443/index.html>
2. test https websockets (WARN must accept certificates in browser to work)
   1. https://localhost:8443/static/ws.html
3. `websocat -k wss://192.168.1.84:8443/ws/`

### Client

```shell
$ cd client
$ pnpm i
$ pnpm dev
  ➜  Local:   http://localhost:5173/
```

