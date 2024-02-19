FROM ubuntu:22.04
ARG USERNAME
ENV TZ=Asia/Tokyo

RUN useradd -m -d /home/$USERNAME -s /bin/bash $USERNAME && \
    apt update && \
    apt upgrade -y && \
    apt install curl -y && \
    apt install unzip -y && \
    apt install git -y && \
    apt install sudo && \
    apt install nodejs npm -y && \
    npm install -g n && \
    n lts && \
    apt purge nodejs npm -y && \
    apt autoremove -y

RUN npm install -g pnpm
    
RUN echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER $USERNAME

RUN true && \
    # Install for frontend 
    # curl -fsSL https://bun.sh/install | bash -s "bun-v1.0.26" && \
    # Install for backend
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "1.75.0" --no-modify-path