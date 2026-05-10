
# Installation Guidelines
## Rust Installation Process
**1. System update and essential tools install:**
```
sudo apt update && sudo apt upgrade -y
sudo apt install curl build-essential -y
sudo apt install php-dev
```

**2. Rust install:**
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
    
**3. Environment setup:**
    ``` 
    source "$HOME/.cargo/env" 
    ENV PATH="/home/app/.cargo/bin:${PATH}" # For Docker container
    ```
    
**4. Installation verify:**
* **Compiler:** ``` rustc --version ```
* **Package Manager:** ``` cargo --version ```

## Create extestion and build

* **Create library:** 
    ```cargo new loan_calc --lib```
    
* **Build library:**
    ``` cargo build --release ```

## PHP.ini update
Add `extension=/var/www/html/target/release/libloan_calc.so` to php.ini

**Restart PHP:**
```
    sudo systemctl restart php8.4-fpm
    sudo service apache2 restart
```

## ext-php-rs

### Bindings and abstractions for the Zend API to build PHP extensions natively in Rust.<br>
     
1. [Documentation](https://docs.rs/ext-php-rs)
2. [Guide](https://ext-php.rs)


## Docker 

**1. Build the Docker image:**
```
docker build -t php-ext-rust .
```
**2. Run the Docker container:**
```
docker run -d -p 8085:80 --name php-ext-rust-container php-ext-rust
docker run --rm -d -p 8085:80 --name php-ext-rust-container php-ext-rust
docker compose build
docker compose up -d
docker compose up --build -d
docker compose down
```
**3. Docker container exec into bash:**
```
docker exec -it php-ext-rust-container bash
```
**4. Access the application:**
```http://localhost:8085
``` 
**5. Inspect the Docker container:**
```
docker inspect -f '{{.State.Pid}}' php-ext-rust-container | xargs kill
docker logs -f php-extension-by-rust-app

``` 
