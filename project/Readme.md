# sewe

1. Запускам Redis сервер 
```shell
docker-compose up -d
```

3. Создаем конфигурационный файл в домашней директрии
```shell
touch ~/.config/project.toml
```
и содержимое
```toml
course_pattern = "Rust"
source_file = "/tmp/ttt.html"
redis_url = "redis://127.0.0.1:6379"
```

4. Скачиваем расписание для  загрузки в БД и помещаем егопод именем совпадающим с конфигом (source_file)

5. Запускаем сервер
```shell
./otus_schedule_bot
```
6. Регистрируем [хух для телегам бота](https://telegram-bot-sdk.readme.io/reference/setwebhook)
7. Произойдет при запуске  парсинг файла с расписанием
8. Запуститься сервер, обрабатывающий 3 команды

```shell
/help — Помощь
/start — Старт
/schedule — Информация о группах
```

![pict](../docs/Peek%202024-01-01%2013-13.gif)