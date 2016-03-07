FROM ubuntu

RUN apt-get update
RUN apt-get install -y curl
RUN curl -sSf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN sh rustup.sh -y
RUN apt-get install -y libcgroup-dev
RUN apt-get install -y build-essential