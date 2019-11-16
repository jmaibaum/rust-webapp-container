FROM scratch

ADD target/x86_64-unknown-linux-musl/release/server /
ADD target/deploy/frontend.js /static/
ADD target/deploy/frontend.wasm /static/
ADD target/deploy/index.html /static/
EXPOSE 8000

CMD ["/server"]
