lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Twój pulpit"),
        ("desk_tip", "Aby połaczyć się z tym urządzeniem należy użyć tego ID i hasła."),
        ("Password", "Hasło"),
        ("Ready", "Gotowe"),
        ("Established", "Nawiązano"),
        ("connecting_status", "Status połączenia"),
        ("Enable Service", "Włącz usługę"),
        ("Start Service", "Uruchom usługę"),
        ("Service is running", "Usługa uruchomiona"),
        ("Service is not running", "Usługa nie jest uruchomiona"),
        ("not_ready_status", "Brak gotowości"),
        ("Control Remote Desktop", "Kontroluj zdalny pulpit"),
        ("Transfer File", "Transfer plików"),
        ("Connect", "Połącz"),
        ("Recent Sessions", "Ostatnie sesje"),
        ("Address Book", "Książka adresowa"),
        ("Confirmation", "Potwierdzenie"),
        ("TCP Tunneling", "Tunelowanie TCP"),
        ("Remove", "Usuń"),
        ("Refresh random password", "Odśwież losowe hasło"),
        ("Set your own password", "Ustaw własne hasło"),
        ("Enable Keyboard/Mouse", "Włącz klawiaturę/mysz"),
        ("Enable Clipboard", "Włącz schowek"),
        ("Enable File Transfer", "Włącz transfer pliku"),
        ("Enable TCP Tunneling", "Włącz tunelowanie TCP"),
        ("IP Whitelisting", "Biała lista IP"),
        ("ID/Relay Server", "Serwer ID/Pośredniczący"),
        ("Import Server Conf", "Importuj konfigurację serwera"),
        ("Import server configuration successfully", "Importowanie konfiguracji serwera powiodło się"),
        ("Invalid server configuration", "Nieprawidłowa konfiguracja serwera"),
        ("Clipboard is empty", "Schowek jest pusty"),
        ("Stop service", "Zatrzymaj usługę"),
        ("Change ID", "Zmień ID"),
        ("Website", "Strona internetowa"),
        ("About", "O"),
        ("Mute", "Wycisz"),
        ("Audio Input", "Wejście audio"),
        ("Enhancements", "Ulepszenia"),
        ("Hardware Codec", "Kodek sprzętowy"),
        ("Adaptive Bitrate", "Adaptacyjny bitrate"),
        ("ID Server", "Serwer ID"),
        ("Relay Server", "Serwer pośredniczący"),
        ("API Server", "Serwer API"),
        ("invalid_http", "Nieprawidłowy żądanie http"),
        ("Invalid IP", "Nieprawidłowe IP"),
        ("id_change_tip", "Nowy ID może być złożony z małych i dużych liter a-zA-z, cyfry 0-9 oraz _ (podkreślenie). Pierwszym znakiem powinna być litera a-zA-Z, a całe ID powinno składać się z 6 do 16 znaków."),
        ("Invalid format", "Nieprawidłowy format"),
        ("server_not_support", "Serwer nie obsługuje tej funkcji"),
        ("Not available", "Niedostępne"),
        ("Too frequent", "Zbyt często"),
        ("Cancel", "Anuluj"),
        ("Skip", "Pomiń"),
        ("Close", "Zamknij"),
        ("Retry", "Ponów"),
        ("OK", "OK"),
        ("Password Required", "Wymagane jest hasło"),
        ("Please enter your password", "Wpisz proszę twoje hasło"),
        ("Remember password", "Zapamiętaj hasło"),
        ("Wrong Password", "Błędne hasło"),
        ("Do you want to enter again?", "Czy chcesz wprowadzić ponownie?"),
        ("Connection Error", "Błąd połączenia"),
        ("Error", "Błąd"),
        ("Reset by the peer", "Połączenie zresetowanie przez peer"),
        ("Connecting...", "Łączenie..."),
        ("Connection in progress. Please wait.", "Trwa łączenie. Proszę czekać."),
        ("Please try 1 minute later", "Spróbuj za minutę"),
        ("Login Error", "Błąd logowania"),
        ("Successful", "Sukces"),
        ("Connected, waiting for image...", "Połączono, czekam na obraz..."),
        ("Name", "Nazwa"),
        ("Type", "Typ"),
        ("Modified", "Zmodyfikowany"),
        ("Size", "Rozmiar"),
        ("Show Hidden Files", "Pokaż ukryte pliki"),
        ("Receive", "Odbierz"),
        ("Send", "Wyślij"),
        ("Refresh File", "Odśwież plik"),
        ("Local", "Lokalny"),
        ("Remote", "Zdalny"),
        ("Remote Computer", "Zdalny komputer"),
        ("Local Computer", "Lokalny komputer"),
        ("Confirm Delete", "Potwierdź usunięcie"),
        ("Delete", "Usuń"),
        ("Properties", "Właściwości"),
        ("Multi Select", "Wielokrotny wybór"),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", "Pusty katalog"),
        ("Not an empty directory", "Katalog nie jest pusty"),
        ("Are you sure you want to delete this file?", "Czy na pewno chcesz usunąć ten plik?"),
        ("Are you sure you want to delete this empty directory?", "Czy na pewno chcesz usunać ten pusty katalog?"),
        ("Are you sure you want to delete the file of this directory?", "Czy na pewno chcesz usunąć pliki z tego katalog?"),
        ("Do this for all conflicts", "Zrób to dla wszystkich konfliktów"),
        ("This is irreversible!", "To jest nieodwracalne!"),
        ("Deleting", "Usuwanie"),
        ("files", "pliki"),
        ("Waiting", "Oczekiwanie"),
        ("Finished", "Zakończono"),
        ("Speed", "Prędkość"),
        ("Custom Image Quality", "Niestandardowa jakość obrazu"),
        ("Privacy mode", "Tryb prywatny"),
        ("Block user input", "Blokuj peryferia użytkownika"),
        ("Unblock user input", "Odblokuj peryferia użytkownika"),
        ("Adjust Window", "Dostosuj okno"),
        ("Original", "Oryginalny"),
        ("Shrink", "Zmniejsz"),
        ("Stretch", "Rozciągnij"),
        ("Scrollbar", "Pasek przewijania"),
        ("ScrollAuto", "Przewijanie automatyczne"),
        ("Good image quality", "Dobra jakość obrazu"),
        ("Balanced", "Zrównoważony"),
        ("Optimize reaction time", "Zoptymalizuj czas reakcji"),
        ("Custom", ""),
        ("Show remote cursor", "Pokazuj zdalny kursor"),
        ("Show quality monitor", "Pokazuj jakość monitora"),
        ("Disable clipboard", "Wyłącz schowek"),
        ("Lock after session end", "Zablokuj po zakończeniu sesji"),
        ("Insert", "Wstaw"),
        ("Insert Lock", "Wstaw blokadę"),
        ("Refresh", "Odśwież"),
        ("ID does not exist", "ID nie istnieje"),
        ("Failed to connect to rendezvous server", "Nie udało się połączyć z serwerem połączeń"),
        ("Please try later", "Spróbuj później"),
        ("Remote desktop is offline", "Zdalny pulpit jest offline"),
        ("Key mismatch", "Niezgodność klucza"),
        ("Timeout", "Przekroczenie czasu"),
        ("Failed to connect to relay server", "Nie udało się połączyć z serwerem pośredniczącym"),
        ("Failed to connect via rendezvous server", "Nie udało się połączyć przez serwer połączeń"),
        ("Failed to connect via relay server", "Nie udało się połączyć przez serwer pośredniczący"),
        ("Failed to make direct connection to remote desktop", "Nie udało się nawiązać bezpośredniego połączenia z pulpitem zdalnym"),
        ("Set Password", "Ustaw hasło"),
        ("OS Password", "Hasło systemu operacyjnego"),
        ("install_tip", "RustDesk może nie działać poprawnie na maszynie zdalnej z przyczyn związanych z UAC. W celu uniknięcią problemów z UAC, kliknij poniższy przycisk by zainstalować RustDesk w swoim systemie."),
        ("Click to upgrade", "Kliknij, aby zaktualizować (upgrade)"),
        ("Click to download", "Kliknij, aby pobrać"),
        ("Click to update", "Kliknij, aby zaktualizować (update)"),
        ("Configure", "Konfiguruj"),
        ("config_acc", "Konfiguracja konta"),
        ("config_screen", "Konfiguracja ekranu"),
        ("Installing ...", "Instalowanie..."),
        ("Install", "Zainstaluj"),
        ("Installation", "Instalacja"),
        ("Installation Path", "Ścieżka instalacji"),
        ("Create start menu shortcuts", "Utwórz skrót w menu"),
        ("Create desktop icon", "Utwórz skrót na pulpicie"),
        ("agreement_tip", "Wskazówki do umowy/zgody"),
        ("Accept and Install", "Akceptuj i zainstaluj"),
        ("End-user license agreement", "Umowa licencyjna użytkownika końcowego"),
        ("Generating ...", "Trwa generowanie..."),
        ("Your installation is lower version.", "Twoja instalacja jest w niższej wersji"),
        ("not_close_tcp_tip", "Podczas korzystanie z tunelowanie, nie zamykaj tego okna."),
        ("Listening ...", "Nasłuchiwanie..."),
        ("Remote Host", "Host zdalny"),
        ("Remote Port", "Port zdalny"),
        ("Action", "Akcja"),
        ("Add", "Dodaj"),
        ("Local Port", "Lokalny port"),
        ("Local Address", "Lokalny adres"),
        ("Change Local Port", "Zmień lokalny port"),
        ("setup_server_tip", "W celu uzyskania szybszego połączenia, skorzystaj z własnego serwera połączeń."),
        ("Too short, at least 6 characters.", "Za krótkie, min. 6 znaków"),
        ("The confirmation is not identical.", "Potwierdzenie nie jest identyczne."),
        ("Permissions", "Uprawnienia"),
        ("Accept", "Akceptuj"),
        ("Dismiss", "Odrzuć"),
        ("Disconnect", "Rozłącz"),
        ("Allow using keyboard and mouse", "Zezwalaj na używanie klawiatury i myszy"),
        ("Allow using clipboard", "Zezwalaj na używanie schowka"),
        ("Allow hearing sound", "Zezwól na transmisję audio"),
        ("Allow file copy and paste", "Zezwalaj na kopiowanie i wklejanie plików"),
        ("Connected", "Połączony"),
        ("Direct and encrypted connection", "Połączenie bezpośrednie i szyfrowane"),
        ("Relayed and encrypted connection", "Połączenie pośrednie i szyfrowane"),
        ("Direct and unencrypted connection", "Połączenie bezpośrednie i nieszyfrowane"),
        ("Relayed and unencrypted connection", "Połączenie pośrednie i nieszyfrowane"),
        ("Enter Remote ID", "Wprowadź zdalne ID"),
        ("Enter your password", "Wprowadź hasło"),
        ("Logging in...", "Trwa logowanie..."),
        ("Enable RDP session sharing", "Włącz udostępnianie sesji RDP"),
        ("Auto Login", "Automatyczne logowanie"),
        ("Enable Direct IP Access", "Włącz Bezpośredni dostęp IP"),
        ("Rename", "Zmień nazwę"),
        ("Space", "Przestrzeń"),
        ("Create Desktop Shortcut", "Utwórz skrót na pulpicie"),
        ("Change Path", "Zmień ścieżkę"),
        ("Create Folder", "Utwórz folder"),
        ("Please enter the folder name", "Wpisz nazwę folderu"),
        ("Fix it", "Napraw to"),
        ("Warning", "Ostrzeżenie"),
        ("Login screen using Wayland is not supported", "Ekran logowania korzystający z Wayland nie jest obsługiwany"),
        ("Reboot required", "Wymagany ponowne uruchomienie"),
        ("Unsupported display server ", "Nieobsługiwany serwer wyświetlania "),
        ("x11 expected", "Wymagany jest X11"),
        ("Port", "Port"),
        ("Settings", "Ustawienia"),
        ("Username", "Nazwa użytkownika"),
        ("Invalid port", "Nieprawidłowy port"),
        ("Closed manually by the peer", "Połączenie zakończone ręcznie przez peer"),
        ("Enable remote configuration modification", "Włącz zdalną modyfikację konfiguracji"),
        ("Run without install", "Uruchom bez instalacji"),
        ("Always connected via relay", "Zawsze połączony pośrednio"),
        ("Always connect via relay", "Zawsze łącz pośrednio"),
        ("whitelist_tip", "Podpowiedź do białej listy"),
        ("Login", "Zaloguj"),
        ("Logout", "Wyloguj"),
        ("Tags", "Tagi"),
        ("Search ID", "Szukaj ID"),
        ("Current Wayland display server is not supported", "Obecny serwer wyświetlania Wayland nie jest obsługiwany"),
        ("whitelist_sep", "Seperator białej listy"),
        ("Add ID", "Dodaj ID"),
        ("Add Tag", "Dodaj Tag"),
        ("Unselect all tags", "Odznacz wszystkie tagi"),
        ("Network error", "Błąd sieci"),
        ("Username missed", "Brak użytkownika"),
        ("Password missed", "Brak hasła"),
        ("Wrong credentials", "Błędne dane uwierzytelniające"),
        ("Edit Tag", "Edytuj tag"),
        ("Unremember Password", "Zapomnij hasło"),
        ("Favorites", "Ulubione"),
        ("Add to Favorites", "Dodaj do ulubionych"),
        ("Remove from Favorites", "Usuń z ulubionych"),
        ("Empty", "Pusty"),
        ("Invalid folder name", "Błędna nazwa folderu"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Nazwa hosta"),
        ("Discovered", "Wykryte"),
        ("install_daemon_tip", "Podpowiedź instalacji daemona"),
        ("Remote ID", "Zdalne ID"),
        ("Paste", "Wklej"),
        ("Paste here?", "Wkleić tu?"),
        ("Are you sure to close the connection?", "Czy na pewno chcesz zamknąć połączenie?"),
        ("Download new version", "Pobierz nową wersję"),
        ("Touch mode", "Tryb dotykowy"),
        ("Mouse mode", "Tryb myszy"),
        ("One-Finger Tap", "Dotknij jednym palcem"),
        ("Left Mouse", "Lewy klik myszy"),
        ("One-Long Tap", "Przytrzymaj jednym palcem"),
        ("Two-Finger Tap", "Dotknij dwoma palcami"),
        ("Right Mouse", "Prawy klik myszy"),
        ("One-Finger Move", "Ruch jednym palcem"),
        ("Double Tap & Move", "Dotknij dwukrotnie i przesuń"),
        ("Mouse Drag", "Przeciągnij myszą"),
        ("Three-Finger vertically", "Trzy palce wertykalnie"),
        ("Mouse Wheel", "Skrol myszy"),
        ("Two-Finger Move", "Ruch dwoma palcami"),
        ("Canvas Move", "Ruch ekranu"),
        ("Pinch to Zoom", "Uszczypnij, aby powiększyć"),
        ("Canvas Zoom", "Powiększanie ekranu"),
        ("Reset canvas", "Reset ekranu"),
        ("No permission of file transfer", "Brak uprawnień na przesyłanie plików"),
        ("Note", "Notatka"),
        ("Connection", "Połączenie"),
        ("Share Screen", "Udostępnij ekran"),
        ("CLOSE", "Zamknij"),
        ("OPEN", "Otwórz"),
        ("Chat", "Czat"),
        ("Total", "Łącznie"),
        ("items", "elementy"),
        ("Selected", "Zaznaczone"),
        ("Screen Capture", "Przechwytywanie ekranu"),
        ("Input Control", "Kontrola wejścia"),
        ("Audio Capture", "Przechwytywanie dźwięku"),
        ("File Connection", "File Connection"),
        ("Screen Connection", "Screen Connection"),
        ("Do you accept?", "Akceptujesz?"),
        ("Open System Setting", "Otwórz ustawienia systemowe"),
        ("How to get Android input permission?", "Jak uzyskać uprawnienia do wprowadzania danych w systemie Android?"),
        ("android_input_permission_tip1", "android_input_permission_tip1"),
        ("android_input_permission_tip2", "android_input_permission_tip2"),
        ("android_new_connection_tip", "android_new_connection_tip"),
        ("android_service_will_start_tip", "android_service_will_start_tip"),
        ("android_stop_service_tip", "android_stop_service_tip"),
        ("android_version_audio_tip", "android_version_audio_tip"),
        ("android_start_service_tip", "android_start_service_tip"),
        ("Account", "Konto"),
        ("Overwrite", "Nadpisz"),
        ("This file exists, skip or overwrite this file?", "Ten plik istnieje, pominąć czy nadpisać ten plik?"),
        ("Quit", "Zrezygnuj"),
        ("doc_mac_permission", "doc_mac_permission"),
        ("Help", "Pomoc"),
        ("Failed", "Niepowodzenie"),
        ("Succeeded", "Udało się"),
        ("Someone turns on privacy mode, exit", "Ktoś włącza tryb prywatności, wyjdź"),
        ("Unsupported", "Niewspierane"),
        ("Peer denied", "Odmowa dostępu"),
        ("Please install plugins", "Zainstaluj plugin"),
        ("Peer exit", "Wyjście peer"),
        ("Failed to turn off", "Nie udało się wyłączyć"),
        ("Turned off", "Wyłączony"),
        ("In privacy mode", "Uruchom tryb prywatności"),
        ("Out privacy mode", "Opuść tryb prywatności"),
        ("Language", "Język"),
        ("Keep RustDesk background service", "Zachowaj usługę w tle RustDesk"),
        ("Ignore Battery Optimizations", "Ignoruj optymalizację baterii"),
        ("android_open_battery_optimizations_tip", "android_open_battery_optimizations_tip"),
        ("Connection not allowed", "Połączenie niedozwolone"),
        ("Legacy mode", "Tryb wstecznej-kompatybilności (legacy)"),
        ("Map mode", "Tryb mapowania"),
        ("Translate mode", "Tryb translacji"),
        ("Use temporary password", "Użyj tymczasowego hasła"),
        ("Use permanent password", "Użyj hasła permanentnego"),
        ("Use both passwords", "Użyj obu haseł"),
        ("Set permanent password", "Ustaw hasło permanentne"),
        ("Set temporary password length", "Ustaw długość hasła tymczasowego"),
        ("Enable Remote Restart", "Włącz Zdalne Restartowanie"),
        ("Allow remote restart", "Zezwól na zdalne restartowanie"),
        ("Restart Remote Device", "Zrestartuje Zdalne Urządzenie"),
        ("Are you sure you want to restart", "Czy na pewno uruchomić ponownie"),
        ("Restarting Remote Device", "Trwa restartowanie Zdalnego Urządzenia"),
        ("remote_restarting_tip", "Trwa ponownie uruchomienie zdalnego urządzenia, zamknij ten komunikat i ponownie nawiąż za chwilę połączenie używając hasła permanentnego"),
        ("Copied", "Skopiowano"),
        ("Exit Fullscreen", "Wyłączyć tryb pełnoekranowy"),
        ("Fullscreen", "Pełny ekran"),
        ("Mobile Actions", "Działania mobilne"),
        ("Select Monitor", "Wybierz Monitor"),
        ("Control Actions", "Działania kontrolne"),
        ("Display Settings", "Ustawienia wyświetlania"),
        ("Ratio", "Proporcje"),
        ("Image Quality", "Jakość obrazu"),
        ("Scroll Style", "Styl przewijania"),
        ("Show Menubar", "Pokaż pasek menu"),
        ("Hide Menubar", "Ukryj pasek menu"),
        ("Direct Connection", "Połącznie Bezpośrednie"),
        ("Relay Connection", "Połączenie Pośrednie"),
        ("Secure Connection", "Połączenie Bezpieczne"),
        ("Insecure Connection", "Połączenie Niebezpieczne"),
        ("Scale original", "Skaluj oryginalnie"),
        ("Scale adaptive", "Skaluj adaptacyjnie"),
        ("General", "Ogólne"),
        ("Security", "Zabezpieczenia"),
        ("Account", "Konto"),
        ("Theme", "Motyw"),
        ("Dark Theme", "Ciemny motyw"),
        ("Dark", "Ciemny"),
        ("Light", "Jasny"),
        ("Follow System", "Zgodne z systemem"),
        ("Enable hardware codec", "Włącz wsparcie sprzętowe dla kodeków"),
        ("Unlock Security Settings", "Odblokuj Ustawienia Zabezpieczeń"),
        ("Enable Audio", "Włącz Dźwięk"),
        ("Temporary Password Length", "Długość hasła tymaczowego"),
        ("Unlock Network Settings", "Odblokuj ustawienia Sieciowe"),
        ("Server", "Serwer"),
        ("Direct IP Access", "Bezpośredni Adres IP"),
        ("Proxy", "Proxy"),
        ("Port", "Port"),
        ("Apply", "Zastosuj"),
        ("Disconnect all devices?", "Czy rozłączyć wszystkie urządzenia?"),
        ("Clear", "Wyczyść"),
        ("Audio Input Device", "Urządzenie wejściowe Audio"),
        ("Deny remote access", "Zabroń dostępu zdalnego"),
        ("Use IP Whitelisting", "Użyj białej listy IP"),
        ("Network", "Sieć"),
        ("Enable RDP", "Włącz RDP"),
        ("Pin menubar", "Przypnij pasek menu"),
        ("Unpin menubar", "Odepnij pasek menu"),
        ("Recording", "Trwa nagrywanie"),
        ("Directory", "Katalog"),
        ("Automatically record incoming sessions", "Automatycznie nagrywaj sesje przychodzące"),
        ("Change", "Zmień"),
        ("Start session recording", "Zacznij nagrywać sesję"),
        ("Stop session recording", "Zatrzymaj nagrywanie sesji"),
        ("Enable Recording Session", "Włącz Nagrywanie Sesji"),
        ("Allow recording session", "Zezwól na nagrywanie sesji"),
        ("Enable LAN Discovery", "Włącz Wykrywanie LAN"),
        ("Deny LAN Discovery", "Zablokuj Wykrywanie LAN"),
        ("Write a message", "Napisz wiadomość"),
        ("Prompt", "Monit"),
        ("elevation_prompt", "Monit o podwyższeniu uprawnień"),
        ("uac_warning", "Ostrzeżenie UAC"),
        ("elevated_foreground_window_warning", "Pierwszoplanowe okno ostrzeżenia o podwyższeniu uprawnień"),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Custom", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland wymaga Ubuntu 21.04 lub nowszego."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland wymaga wyższej wersji dystrybucji Linuksa. Wypróbuj pulpit X11 lub zmień system operacyjny."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Wybierz ekran do udostępnienia (działaj po stronie równorzędnej)."),
    ].iter().cloned().collect();
}
