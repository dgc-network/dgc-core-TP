# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.19.0
WORKDIR /app
ADD . /app
EXPOSE 3000
RUN cargo build
ENTRYPOINT ["target/debug/rest-api"]