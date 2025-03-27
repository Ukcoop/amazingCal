FROM rust:1.83

RUN rustup target add wasm32-unknown-unknown

# Install trunk (locked for reproducibility)
RUN cargo install trunk --locked

WORKDIR /app
COPY . .

EXPOSE 3000

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "3000"]
