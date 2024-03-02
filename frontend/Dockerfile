FROM node:20.0.0 AS base
ENV TZ=Asia/Tokyo
RUN npm install -g pnpm

FROM base AS dev
ARG UID=1000
ARG GID=1000
RUN groupmod --gid $GID node && \
    usermod --uid $UID node