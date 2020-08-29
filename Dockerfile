FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY ./target/release/email_microservice /usr/local/bin/email_microservice
RUN apt-get update && apt-get install -y
RUN apt-get install curl -y
STOPSIGNAL SIGINT
ENTRYPOINT ["email_microservice"]