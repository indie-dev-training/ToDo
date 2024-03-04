FROM rust:1.76 AS base
ENV TZ=Asia/Tokyo

FROM base AS dev
ARG UID=1000
ARG GID=1000
ARG USERNAME=app
RUN groupadd --gid $GID $USERNAME && \
    useradd --uid $UID --gid $GID -m -d /home/$USERNAME -s /bin/bash $USERNAME
