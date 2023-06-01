# Генератор случайных простых чисел произвольного размера на языке Rust

## Установка

1. Установите пакетный менеджер [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
2. Клонируйте репозиторий:
    ```sh
    git clone https://github.com/lincot/gen-prime
    ```

## Запуск

Введите следующую команду, где `<bits>` – желаемое количество значимых бит:

```sh
cargo run --release <bits>
```

Выведется простое число с указанным количеством значимых бит.
