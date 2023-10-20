# builder
FROM rust:1.75.0 as builder
WORKDIR /usr/src/api
COPY . .
RUN cargo install --path .

# distroless
FROM gcr.io/distroless/cc
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]
