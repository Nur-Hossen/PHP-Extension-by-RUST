FROM php:8.5-fpm

LABEL maintainer="Nur Hossen <nur.hossen@datasoft-bd.com>"

ARG HOST_APP_ROOT_DIR=./codes
ARG WORK_DIR=/var/www/html/

ENV TZ=Asia/Dhaka

WORKDIR $WORK_DIR

RUN apt-get update && apt-get install -y \
    libpng-dev \
    libjpeg62-turbo-dev \
    libfreetype6-dev \
    libxml2-dev \
    libzip-dev \
    libonig-dev \
    curl \
    unzip \
    build-essential \
    clang \
    libclang-dev \
    llvm 

RUN docker-php-ext-configure gd --with-freetype --with-jpeg

# RUN docker-php-ext-install -j$(nproc) gd && \
#     docker-php-ext-install pdo_mysql && \
#     docker-php-ext-install mysqli && \
#     docker-php-ext-install zip && \
#     docker-php-ext-install opcache && \
#     docker-php-source delete

# RUN docker-php-source delete

ENV RUSTUP_HOME=/home/app/.rustup
ENV CARGO_HOME=/home/app/.cargo

# Rust install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Set the PATH environment variable to include Cargo's bin directory
ENV PATH="/home/app/.cargo/bin:${PATH}"
# Verify Rust installation
RUN cargo --version

RUN groupadd -g 1000 app && \
    useradd -u 1000 -ms /bin/bash -g app app
    
RUN chown -R app:app $WORK_DIR
RUN chown -R app:app /home/app/.cargo
USER app

COPY --chown=app:app ./docker/app/php.ini /usr/local/etc/php/php.ini
COPY --chown=app:app $HOST_APP_ROOT_DIR/ $WORK_DIR


EXPOSE 9001

CMD ["php-fpm"]