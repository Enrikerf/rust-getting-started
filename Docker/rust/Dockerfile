FROM rust:1.63

WORKDIR /volume

RUN apt-get update \
    && apt-get install fish git vim openssh-client sudo -y \
    && groupadd -g 1000 container-user \
    && useradd -d /home/container-user -s /bin/bash -u 1000 -g 1000 container-user \
    && mkdir /home/container-user \
    && chown -R container-user:container-user /home/container-user \
    && usermod -aG sudo container-user && echo "container-user:1234" | sudo chpasswd
