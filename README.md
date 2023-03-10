# Blog service on rust (actix-web + diesel)

### Аннотация
Блог сервис представляет из себя два HTTP-API сервиса написанных на Rust.  

Первый - сервис пользователей.  
Имеет эндпоинты: 
1. ```POST /auth/register``` - Регистрация пользователя
1. ```POST /auth/login``` - Авторизация. В качестве ответа возвращается JWT-токен, который также устанавливается в cookies
1. ```POST /auth/logout``` - Удаление JWT-токена из cookies
1. ```GET /users``` - Получение информации о всех пользователях
1. ```GET /users/{id}``` - Получение информации о конкретном пользователе (по id)

Второй - сервис постов.  
Имеет эндпоинты:
1. ```POST /posts``` - Создание поста
1. ```PUT /posts/{id}``` - Изменение поста
1. ```DELETE /posts/{id}``` - Удаление поста
1. ```GET /posts``` - Получение информации о всех постах
1. ```GET /posts/{id}``` - Получение информации о конкретном посте (по id)

_Примечание:_ Все эндпоинты, за исключением ```POST /auth/login``` и ```POST /auth/register```, требуют авторизации

Для упрощенного взаимодействия с API, в корневой директории проекта есть подготовленные запросы для Postman - [Blog service.postman_collection.json](Blog%20service.postman_collection.json)

### Пример запуска
Создайте файл ```.env``` и заполните его значениями согласно примеру из файла ```.env.template```

Далее, выполните команду из корневой директории проекта:
```shell
docker compose up -d --build
```
Чтобы остановить и удалить все сервисы, контейнеры и связанные образы, выполните:
```shell
docker compose down -v --rmi="all"
```

### Тестирование

Тесты и линтер запускаются в github actions на каждый push.

Для локального запуска тестов выполните команды:
```shell
cargo test --manifest-path ./auth_service/Cargo.toml
cargo test --manifest-path ./post_service/Cargo.toml
```

_Примечание:_ Для запуска тестов локально, как и для запуска сервисов вне докера, необходимо создать symlinks для директории common_lib  
Для этого выполните команды:
```shell
ln -rs common_lib/ ./auth_service/src/
ln -rs common_lib/ ./post_service/src/
```






