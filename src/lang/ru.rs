lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Cтатус"),
        ("Your Desktop", "Ваш рабочий стол"),
        ("desk_tip", "Ваш рабочий стол доступен с этим идентификатором и паролем"),
        ("Password", "Пароль"),
        ("Ready", "Готово"),
        ("connecting_status", "Подключение к сети RustDesk.."),
        ("Enable Service", "Включить связанный сервер"),
        ("Start Service", "Запуск связанного сервера"),
        ("Service is not running", "Связанный сервер не запущен"),
        ("not_ready_status", "Не готово. Пожалуйста, проверьте подключение"),
        ("Control Remote Desktop", "Управление удаленным рабочим столом"),
        ("Transfer File", "Передать файл"),
        ("Connect", "Подключить"),
        ("Recent Sessions", "Последние сеансы"),
        ("Address Book", "Адресная книга"),
        ("Confirmation", "Подтверждение"),
        ("TCP Tunneling", "TCP-туннелирование"),
        ("Remove", "Удалить"),
        ("Refresh random password", "Обновить случайный пароль"),
        ("Set your own password", "Установи свой пароль"),
        ("Enable Keyboard/Mouse", "Включить клавиатуру/мышь"),
        ("Enable Clipboard", "Включить буфер обмена"),
        ("Enable File Transfer", "Включить передачу файлов"),
        ("Enable TCP Tunneling", "Включить туннелирование TCP"),
        ("IP Whitelisting", "Список разрешенных IP-адресов"),
        ("ID/Relay Server", "ID/Сервер ретрансляции"),
        ("Stop service", "Остановить сервер"),
        ("Change ID", "Изменить ID"),
        ("Website", "Веб-сайт"),
        ("About", "О RustDesk"),
        ("Mute", "Отключить звук"),
        ("Audio Input", "Аудиовход"),
        ("ID Server", "ID сервер"),
        ("Relay Server", "Сервер ретрансляции"),
        ("API Server", "API сервер"),
        ("invalid_http", "Должен начинаться с http:// или https://"),
        ("Invalid IP", "Неверный IP-адрес"),
        ("id_change_tip", "Допускаются только символы a-z, A-Z, 0-9 и _ (подчеркивание). Первая буква должна быть a-z, A-Z. Длина от 6 до 16"),
        ("Invalid format", "Неверный формат"),
        ("This function is turned off by the server", "Эта функция не предоставляется сервером"),
        ("Not available", "Недоступно"),
        ("Too frequent", "Слишком часто"),
        ("Cancel", "Отменить"),
        ("Skip", "Пропустить"),
        ("Close", "Закрыть"),
        ("Retry", "Попробовать еще раз"),
        ("OK", "ОК"),
        ("Password Required", "Требуется пароль"),
        ("Please enter your password", "Пожалуйста, введите ваш пароль"),
        ("Remember password", "Запомнить пароль"),
        ("Wrong Password", "Неверный пароль"),
        ("Do you want to enter again?", "Вы хотите снова войти?"),
        ("Connection Error", "Ошибка подключения"),
        ("Error", "Ошибка"),
        ("Reset by the peer", "Сброшено узлом"),
        ("Connecting...", "Подключение.."),
        ("Connection in progress. Please wait.", "Подключение. Подождите"),
        ("Please try 1 minute later", "Попробуйте через 1 минуту"),
        ("Login Error", "Ошибка входа"),
        ("Successful", "Успешный"),
        ("Connected, waiting for image...", "Подключено, ожидание изображения.."),
        ("Name", "Имя"),
        ("Type", "Тип"),
        ("Modified", "Изменено"),
        ("Size", "Размер"),
        ("Show Hidden Files", "Показать скрытые файлы"),
        ("Receive", "Получено"),
        ("Send", "Отправить"),
        ("Refresh File", "Обновить файл"),
        ("Local", "Местный"),
        ("Remote", "удаленный"),
        ("Remote Computer", "Удаленный компьютер"),
        ("Local Computer", "Локальный компьютер"),
        ("Confirm Delete", "Подтвердить удаление"),
        ("Delete", "Удалить"),
        ("Properties", "Properties"),
        ("Multi Select", "многоэлементный"),
        ("Empty Directory", "Пустой каталог"),
        ("Not an empty directory", "Не пустой каталог"),
        ("Are you sure you want to delete this file?", "Вы уверены, что хотите удалить этот файл?"),
        ("Are you sure you want to delete this empty directory?", "Вы уверены, что хотите удалить этот пустой каталог?"),
        ("Are you sure you want to delete the file of this directory?", "Вы уверены, что хотите удалить файл из этого каталога?"),
        ("Do this for all conflicts", "Это относится ко всем конфликтам"),
        ("This is irreversible!", "Это необратимо!"),
        ("Deleting", "Удаление"),
        ("files", "файлы"),
        ("Waiting", "Ожидание"),
        ("Finished", "Завершено"),
        ("Speed", "Скорость"),
        ("Custom Image Quality", "Пользовательское качество изображения"),
        ("Privacy mode", "Режим конфиденциальности"),
        ("Block user input", "Блокировать пользовательский ввод"),
        ("Unblock user input", "Разблокировать пользовательский ввод"),
        ("Adjust Window", "Настроить окно"),
        ("Original", "Оригинал"),
        ("Shrink", "Уменьшить"),
        ("Stretch", "Растянуть"),
        ("Good image quality", "Хорошее качество изображения"),
        ("Balanced", "Сбалансированный"),
        ("Optimize reaction time", "Оптимизировать время реакции"),
        ("Custom", "Пользовательский"),
        ("Show remote cursor", "Показать удаленный курсор"),
        ("Disable clipboard", "Отключить буфер обмена"),
        ("Lock after session end", "Блокировка после завершения сеанса"),
        ("Insert", "Вставить"),
        ("Insert Lock", "Установить замок"),
        ("Refresh", "Обновить"),
        ("ID does not exist", "Идентификатор не существует"),
        ("Failed to connect to rendezvous server", "Не удалось подключиться к серверу встречи"),
        ("Please try later", "Пожалуйста, попробуйте позже"),
        ("Remote desktop is offline", "Удаленный рабочий стол отключен"),
        ("Key mismatch", "Несоответствие ключей"),
        ("Timeout", "Тайм-аут"),
        ("Failed to connect to relay server", "Не удалось подключиться к серверу ретрансляции"),
        ("Failed to connect via rendezvous server", "Не удалось подключиться через сервер встречи"),
        ("Failed to connect via relay server", "Не удалось подключиться через сервер ретрансляции"),
        ("Failed to make direct connection to remote desktop", "Не удалось установить прямое подключение к удаленному рабочему столу"),
        ("Set Password", "Установить пароль"),
        ("OS Password", "Пароль операционной системы"),
        ("install_tip", "В некоторых случаях из-за UAC RustDesk может работать некорректно на удаленном узле. Чтобы избежать UAC, нажмите кнопку ниже, чтобы установить RustDesk в системе"),
        ("Click to upgrade", "Нажмите, чтобы обновить"),
        ("Click to download", "Нажмите, чтобы скачать"),
        ("Click to update", "Нажмите, чтобы обновить"),
        ("Configuration Permissions", "Разрешения на настройку"),
        ("Configure", "Настроить"),
        ("config_acc", "Чтобы удаленно управлять своим рабочим столом, вы должны предоставить RustDesk права \"доступа\""),
        ("config_screen", "Для удаленного доступа к рабочему столу вы должны предоставить RustDesk права \"снимок экрана\""),
        ("Installing ...", "Установка..."),
        ("Install", "Установить"),
        ("Installation", "Настройка"),
        ("Installation Path", "Путь установки"),
        ("Create start menu shortcuts", "Создать ярлыки меню \"Пуск\""),
        ("Create desktop icon", "Создать значок на рабочем столе"),
        ("agreement_tip", "Если вы начнете установку, примите лицензионное соглашение"),
        ("Accept and Install", "Принять и установить"),
        ("End-user license agreement", "Лицензионное соглашение с конечным пользователем"),
        ("Generating ...", "Генерация..."),
        ("Your installation is lower version.", "Ваша инсталяция является более ранней версией"),
        ("not_close_tcp_tip", "Не закрывать это окно при использовании туннеля"),
        ("Listening ...", "Прослушивание ..."),
        ("Remote Host", "Удаленная машина"),
        ("Remote Port", "Удаленный порт"),
        ("Action", "Действие"),
        ("Add", "Добавить"),
        ("Local Port", "Локальный порт"),
        ("setup_server_tip", "Для более быстрого подключения настройте свой собственный сервер подключения"),
        ("Too short, at least 6 characters.", "Слишком коротко, не менее 6 символов"),
        ("The confirmation is not identical.", "Подтверждение не идентично"),
        ("Permissions", "Разрешения"),
        ("Accept", "Принять"),
        ("Dismiss", "Отклонить"),
        ("Disconnect", "Отключить"),
        ("Allow using keyboard and mouse", "Разрешить использование клавиатуры и мыши"),
        ("Allow using clipboard", "Разрешить использование буфера обмена"),
        ("Allow hearing sound", "Разрешить слышать звук"),
        ("Allow file transfer", "Разрешить передачу файлов"),
        ("File transfer", "Передача файлов"),
        ("Connected", "Подключено"),
        ("Direct and encrypted connection", "Прямое и шифрованное соединение"),
        ("Relayed and encrypted connection", "Коммутируемое и зашифрованное соединение"),
        ("Direct and unencrypted connection", "Прямое и незашифрованное соединение"),
        ("Relayed and unencrypted connection", "Коммутируемое и незашифрованное соединение"),
        ("Enter Remote ID", "Введите удаленный идентификатор"),
        ("Enter your password", "Введите пароль"),
        ("Logging in...", "Регистрация..."),
        ("Enable RDP session sharing", "Включить общий доступ к сеансу RDP"),
        ("Auto Login", "Автоматический вход (действителен, только если вы установили \"Блокировка после завершения сеанса\""),
        ("Enable Direct IP Access", "Включить прямой IP-доступ"),
        ("Rename", "Переименовать"),
        ("Space", "Место"),
        ("Create Desktop Shortcut", "Создать ярлык на рабочем столе"),
        ("Change Path", "Изменить путь"),
        ("Create Folder", "Создать папку"),
        ("Please enter the folder name", "Пожалуйста, введите имя папки"),
        ("Fix it", "Отремонтировать"),
        ("Warning", "Предупреждение"),
        ("Login screen using Wayland is not supported", "Экран входа в систему с использованием Wayland не поддерживается"),
        ("Reboot required", "Требуется перезагрузка"),
        ("Unsupported display server ", "Неподдерживаемый сервер дисплея"),
        ("x11 expected", "Ожидается X11"),
        ("Port", "Порт"),
        ("Settings", "Предпочтения"),
        ("Username", "Имя пользователя"),
        ("Invalid port", "Неверный порт"),
        ("Closed manually by the peer", "Закрыто узлом вручную"),
        ("Enable remote configuration modification", "Разрешить удаленное изменение конфигурации"),
        ("Run without install", "Запустить без установки"),
        ("Always connected via relay", "Всегда подключен через связанный сервер"),
        ("Always connect via relay", "Всегда подключаться через связанный сервер"),
        ("whitelist_tip", "Только IP-адреса из белого списка могут получить доступ ко мне"),
        ("Login", "Войти"),
        ("Logout", "Выйти"),
        ("Tags", "Ключевые слова"),
        ("Search ID", "Идентификатор поиска"),
        ("Current Wayland display server is not supported", "Текущий сервер отображения Wayland не поддерживается"),
        ("whitelist_sep", "Раздельно запятой, точкой с запятой, пробелом или новой строкой"),
        ("Add ID", "Добавить ID"),
        ("Add Tag", "Добавить ключевое слово"),
        ("Unselect all tags", "Отменить выбор всех тегов"),
        ("Network error", "Ошибка сети"),
        ("Username missed", "Имя пользователя отсутствует"),
        ("Password missed", "Забыли пароль"),
        ("Wrong credentials", "Неверные учетные данные"),
        ("Edit Tag", "Редактировать тег"),
        ("Unremember Password", "Не помнить пароль"),
        ("Favorites", "Избранное"),
        ("Add to Favorites", "Добавить в избранное"),
        ("Remove from Favorites", "Удалить из избранного"),
        ("Empty", "Пусто"),
        ("Invalid folder name", "Недопустимое имя папки"),
        ("Socks5 Proxy", "Прокси-сервер Socks5"),
        ("Hostname", "Имя ПК"),
        ("Discovered", "Найдено"),
        ("install_daemon_tip", "Для запуска при загрузке необходимо установить системную службу"),
        ("Remote ID", "Удаленный идентификатор"),
        ("Paste", "Вставить"),
        ("Paste here?", "Вставить сюда?"),
        ("Are you sure to close the connection?", "Вы уверены, что хотите закрыть соединение?"),
        ("Download new version", "Загрузить новую версию"),
        ("Touch mode", "Сенсорный режим"),
        ("TouchPad mode", "Режим сенсорной панели"),
        ("One-Finger Tap", "Касание одним пальцем"),
        ("Left Mouse", "Левая кнопка мыши"),
        ("One-Long Tap", "Один долгий тап"),
        ("Two-Finger Tap", "Касание двумя пальцами"),
        ("Right Mouse", "Правая мышь"),
        ("One-Finger Move", "Движение одним пальцем"),
        ("Double Tap & Move", "Двойное нажатие и перемещение"),
        ("Mouse Drag", "Перетаскивание мышью"),
        ("Two-Finger vertically", "Двумя пальцами по вертикали"),
        ("Mouse Wheel", "Колесико мыши"),
        ("Two-Finger Move", "Движение двумя пальцами"),
        ("Canvas Move", "Перемещение холста"),
        ("Pinch to Zoom", "Щепотка, чтобы увеличить"),
        ("Canvas Zoom", "Масштаб холста"),
        ("Reset canvas", "Сбросить холст"),
        ("No permission of file transfer", "Нет разрешения на передачу файлов"),
        ("Note", "Примечание"),
        ("Connection", "Связь"),
        ("Share Screen", "Поделиться экраном"),
        ("CLOSE", "БЛИЗКО"),
        ("OPEN", "ОТКРЫТЫМ"),
        ("Chat", "Чат"),
        ("Total", "Всего"),
        ("items", "Предметы"),
        ("Selected", "Выбрано"),
        ("Screen Capture", "Скриншот"),
        ("Mouse Control", "Управление мышью"),
        ("File Transfer", "Передача файлов"),
        ("Audio Capture", "Захват аудио"),
        ("File Connection", "Файловое соединение"),
        ("Screen Connection", "Подключение экрана"),
        ("Do you accept?", "Do you accept?"),
        ("Open System Setting", "Открыть настройки системы"),
        ("How to get Android input permission?", "Как получить разрешение на ввод Android?"),
        ("android_input_permission_tip1", "После получения разрешения на ввод удаленное устройство может управлять этим Android-устройством с помощью мыши."),
        ("android_input_permission_tip2", "Перейдите на следующую страницу системных настроек, найдите и войдите в [Установленные службы], включите службу [RustDesk Input]."),
        ("android_new_connection_tip", "Получен новый запрос на управление, он хочет управлять вашим текущим устройством."),
        ("android_service_will_start_tip", "Включение захвата экрана автоматически запускает службу, позволяя другим устройствам запрашивать соединение с этого устройства."),
        ("android_stop_service_tip", "Закрытие службы автоматически закроет все установленные соединения."),
        ("android_version_audio_tip", "Текущая версия Android не поддерживает захват звука, обновите ее до Android 10 или выше."),
        ("android_start_service_tip", "Коснитесь [Запуск связанного сервера] или ОТКРЫТЬ разрешение [Скриншот], чтобы запустить службу демонстрации экрана."),
    ].iter().cloned().collect();
}
