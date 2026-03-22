FROM rust:latest

WORKDIR /app
COPY . .

CMD ["/bin/bash"]
