# Rust build stage
FROM rust:1.67 as builder
WORKDIR /app
RUN cargo install wasm-pack
COPY pak_parser/Cargo.toml pak_parser/Cargo.lock ./
RUN cargo build --release -p pak_parser

# Node build stage
FROM node:18-slim as frontend
WORKDIR /app
COPY pak_editor/ ./
RUN npm install && npm run build

# Final stage
FROM nginx:1.23-alpine
WORKDIR /usr/share/nginx/html
COPY --from=builder /app/target/wasm32-unknown-unknown/release/ ./wasm/
COPY --from=frontend /app/dist ./
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]