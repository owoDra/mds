FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG NODE_MAJOR=24
ARG RUST_VERSION=1.86.0
ARG PYTHON_VERSION=3.13
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup

# ---------- System packages ----------
RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  curl \
  git \
  git-lfs \
  gh \
  gnupg \
  jq \
  less \
  unzip \
  zip \
  sudo \
  openssh-client \
  bash-completion \
  build-essential \
  pkg-config \
  make \
  procps \
  ripgrep \
  fd-find \
  tree \
  bubblewrap \
  python3 \
  python3-venv \
  python3-pip \
  pipx \
  && rm -rf /var/lib/apt/lists/*

# ---------- JavaScript/TypeScript Dev Environment ----------
RUN mkdir -p /etc/apt/keyrings \
  && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
  | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
  && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_MAJOR}.x nodistro main" \
  > /etc/apt/sources.list.d/nodesource.list \
  && apt-get update \
  && apt-get install -y --no-install-recommends nodejs \
  && rm -rf /var/lib/apt/lists/* \
  && npm install -g npm@latest prettier eslint vitest

# ---------- Rust Dev Environment ----------
RUN curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs \
  | sh -s -- -y --profile minimal --default-toolchain ${RUST_VERSION} \
  && /usr/local/cargo/bin/rustup component add rustfmt clippy

# ---------- Python Dev Environment ----------
ENV UV_TOOL_BIN_DIR=/usr/local/bin
RUN curl -LsSf https://astral.sh/uv/install.sh | env UV_INSTALL_DIR=/usr/local/bin sh \
  && uv python install ${PYTHON_VERSION} \
  && ln -sf "$(uv python find ${PYTHON_VERSION})" /usr/local/bin/python3 \
  && uv tool install --python ${PYTHON_VERSION} ruff \
  && uv tool install --python ${PYTHON_VERSION} pytest

# ---------- OpenCode ----------
RUN npm install -g opencode-ai
RUN npm install -g @openai/codex

RUN mkdir -p \
  /home/vscode/.config/opencode \
  /home/vscode/.local/share/opencode \
  /home/vscode/.cache/opencode \
  /home/vscode/.npm-global \
  /home/vscode/.local/bin \
  /home/vscode/.local/share/opencode \
  /home/vscode/.cache/uv \
  /home/vscode/.cache/pip \
  /home/vscode/.cache/cargo \
  /home/vscode/.cache/opencode \
  && chown -R vscode:vscode /home/vscode

ENV NPM_CONFIG_PREFIX=/home/vscode/.npm-global
ENV UV_CACHE_DIR=/home/vscode/.cache/uv
ENV PIP_CACHE_DIR=/home/vscode/.cache/pip
ENV PIPX_HOME=/home/vscode/.local/pipx
ENV PIPX_BIN_DIR=/home/vscode/.local/bin
ENV PATH=/usr/local/cargo/bin:/home/vscode/.npm-global/bin:/home/vscode/.local/bin:/usr/local/bin:${PATH}

WORKDIR /workspace
