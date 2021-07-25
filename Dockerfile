FROM fedora:34
RUN dnf update -y && dnf clean all -y
WORKDIR /usr/local/bin
COPY ./target/release/email_microservice /usr/local/bin/email_microservice
RUN dnf install curl -y && dnf clean all -y
STOPSIGNAL SIGINT
ENTRYPOINT ["email_microservice"]
