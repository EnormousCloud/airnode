FROM debian
RUN apt-get update && apt-get install -y ca-certificates clang librocksdb-dev
EXPOSE 8000
WORKDIR /app/
VOLUME /app/_data
VOLUME /app/_www
COPY ./target/release/airnode-ui-server /app/airnode-ui-server
ENV LOG_LEVEL=airnode_ui_server=debug,info
CMD ["/app/airnode-ui-server", "server", "--no-sync"]
