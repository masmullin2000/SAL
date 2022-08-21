FROM fedora AS rust-build
RUN dnf update -y && dnf install gcc -y && curl https://sh.rustup.rs -sSf | bash -s -- -y
COPY src web_project/src/
COPY benches web_project/benches/
COPY Cargo.toml web_project/
RUN cd web_project && bash -c '. $HOME/.cargo/env && cargo build -r && ldd target/release/api_actix_web'

FROM scratch 
COPY --from=rust-build web_project/target/release/api_actix_web /app
COPY --from=rust-build /lib64/libgcc_s.so.1 /lib64/
COPY --from=rust-build /lib64/libm.so.6 /lib64/
COPY --from=rust-build /lib64/libc.so.6 /lib64/
COPY --from=rust-build /lib64/ld-linux-x86-64.so.2 /lib64/
CMD ["/app"]
