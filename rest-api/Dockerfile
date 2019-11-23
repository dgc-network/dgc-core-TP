# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM ubuntu:xenial

RUN apt-get update \
 && apt-get install -y \
 curl \
 gcc \
 libssl-dev \
 libzmq3-dev \
 pkg-config \
 unzip

RUN \
 if [ ! -z $HTTP_PROXY ] && [ -z $http_proxy ]; then \
  http_proxy=$HTTP_PROXY; \
 fi; \
 if [ ! -z $HTTPS_PROXY ] && [ -z $https_proxy ]; then \
  https_proxy=$HTTPS_PROXY; \
 fi; \
 if [ ! -z $http_proxy ]; then \
  http_proxy_host=$(printf $http_proxy | sed 's|http.*://\(.*\):\(.*\)$|\1|');\
  http_proxy_port=$(printf $http_proxy | sed 's|http.*://\(.*\):\(.*\)$|\2|');\
  mkdir -p $HOME/.cargo \
  && echo "[http]" >> $HOME/.cargo/config \
  && echo 'proxy = "'$http_proxy_host:$http_proxy_port'"' >> $HOME/.cargo/config \
  && cat $HOME/.cargo/config; \
 fi;

RUN curl -OLsS https://github.com/google/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip \
 && unzip protoc-3.5.1-linux-x86_64.zip -d protoc3 \
 && rm protoc-3.5.1-linux-x86_64.zip

RUN curl https://sh.rustup.rs -sSf > /usr/bin/rustup-init \
 && chmod +x /usr/bin/rustup-init \
 && rustup-init -y

ENV PATH=$PATH:/project/dgc-core/processor/bin:/protoc3/bin:/root/.cargo/bin \
    CARGO_INCREMENTAL=0

RUN rustup component add rustfmt-preview


WORKDIR /project/dgc-core/rest-api

###########################################################################
# Below lines are workaround to avoid rebuilding dependencies every time 
# during docker build

# create a new empty shell project
RUN USER=root cargo new --bin my-project
WORKDIR /project/dgc-core/rest-api/my-project

# copy over your manifests 
#(context passed from docker yaml file in parent directory)
COPY ./rest-api/Cargo.lock Cargo.lock
COPY ./rest-api/Cargo.toml Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./rest-api/src ./src
###########################################################################

WORKDIR /project/dgc-core/rest-api

#CMD cd rest-api \
CMD echo "\033[0;32m--- Building dgc-core-rest-api ---\n\033[0m" \
 && rm -rf ./bin/ \
 && mkdir -p ./bin/ \
 && cargo build --release \
 && cp ./target/release/dgc-core-rest-api ./bin/dgc-core-rest-api \
 && cargo run --release --bin dgc-core-rest-api -- -v -C tcp://validator:4004 \
 && tail -f /dev/null
