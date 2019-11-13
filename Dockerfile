FROM scratch

ADD target/x86_64-unknown-linux-musl/release/webapp /
ADD target/deploy/frontend.js /
ADD target/deploy/frontend.wasm /
ADD target/deploy/index.html /
EXPOSE 8000

CMD ["/webapp"]
