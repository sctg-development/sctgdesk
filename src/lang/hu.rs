lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Státusz"),
        ("Your Desktop", "A te asztalod"),
        ("desk_tip", "Az asztalod ezzel az ID-vel, és jelszóval érhető el."),
        ("Password", "Jelszó"),
        ("Ready", "Kész"),
        ("Established", "Létrejött"),
        ("connecting_status", "Kapcsolódás a RustDesk hálózatához..."),
        ("Enable Service", "A szolgáltatás bekapcsolása"),
        ("Start Service", "Szolgáltatás Elindítása"),
        ("Service is running", "A szolgáltatás fut"),
        ("Service is not running", "A szolgáltatás nem fut"),
        ("not_ready_status", "A RustDesk nem áll készen. Kérlek nézd meg a hálózati beállításaidat."),
        ("Control Remote Desktop", "Távoli Asztal Kontrollálása"),
        ("Transfer File", "Fájl Transzfer"),
        ("Connect", "Kapcsolódás"),
        ("Recent Sessions", "Korábbi Sessionök"),
        ("Address Book", "Címköny"),
        ("Confirmation", "Megerősít"),
        ("TCP Tunneling", "TCP Tunneling"),
        ("Remove", "Eltávolít"),
        ("Refresh random password", "Véletlenszerű jelszó frissítése"),
        ("Set your own password", "Saját jelszó beállítása"),
        ("Enable Keyboard/Mouse", "Billentyűzet/Egér bekapcsolása"),
        ("Enable Clipboard", "Megosztott vágólap bekapcsolása"),
        ("Enable File Transfer", "Fájl transzer bekapcsolása"),
        ("Enable TCP Tunneling", "TCP Tunneling bekapcsolása"),
        ("IP Whitelisting", "IP Fehérlista"),
        ("ID/Relay Server", "ID/Relay Szerver"),
        ("Stop service", "Szolgáltatás Kikapcsolása"),
        ("Change ID", "ID Megváltoztatása"),
        ("Website", "Weboldal"),
        ("About", "Rólunk: "),
        ("Mute", "Némítás"),
        ("Audio Input", "Audo Bemenet"),
        ("Enhancements", "Javítások"),
        ("Hardware Codec", "Hardware Kodek"),
        ("Adaptive Bitrate", "Adaptív Bitrate"),
        ("ID Server", "ID Szerver"),
        ("Relay Server", "Relay Szerver"),
        ("API Server", "API Szerver"),
        ("invalid_http", "A címnek mindenképpen http(s)://-el kell kezdődnie."),
        ("Invalid IP", "A megadott íp cím helytelen."),
        ("id_change_tip", "Csak a-z, A-Z, 0-9 csoportokba tartozó karakterek, illetve a _ karakter van engedélyezve. Az első karakternek mindenképpen a-z, A-Z csoportokba kell esnie. Az ID hosszúsága 6-tól, 16 karakter."),
        ("Invalid format", "Érvénytelen formátum"),
        ("server_not_support", "Még nem támogatott a szerver által"),
        ("Not available", "Nem érhető el"),
        ("Too frequent", "Túl gyakori"),
        ("Cancel", "Mégsem"),
        ("Skip", "Kihagy"),
        ("Close", "Bezár"),
        ("Retry", "Újrapróbálkozás"),
        ("OK", "OK"),
        ("Password Required", "A jelszó megadása kötelező"),
        ("Please enter your password", "Kérlek írd be a jelszavad"),
        ("Remember password", "Kérlek emlékezz a jelszóra"),
        ("Wrong Password", "Hibás jelszó"),
        ("Do you want to enter again?", "Újra szeretnéd próbálni?"),
        ("Connection Error", "Kapcsolódási Hiba"),
        ("Error", "Hiba"),
        ("Reset by the peer", "A kapcsolatot alaphelyzetbe állt"),
        ("Connecting...", "Kapcsolódás..."),
        ("Connection in progress. Please wait.", "A kapcsolódás folyamatban van. Kérlek várj."),
        ("Please try 1 minute later", "Kérlek próbáld újra 1 perc múlva."),
        ("Login Error", "Belépési Hiba"),
        ("Successful", "Sikeres"),
        ("Connected, waiting for image...", "Kapcsolódva, várakozás a képre..."),
        ("Name", "Név"),
        ("Type", "Fajta"),
        ("Modified", "Módosított"),
        ("Size", "Méret"),
        ("Show Hidden Files", "Rejtett Fájlok Mutatása"),
        ("Receive", "Kapni"),
        ("Send", "Küldeni"),
        ("Refresh File", "Fájlok Frissítése"),
        ("Local", "Lokális"),
        ("Remote", "Távoli"),
        ("Remote Computer", "Távoli Számítógép"),
        ("Local Computer", "Lokális Számítógép"),
        ("Confirm Delete", "Törlés Megerősítése"),
        ("Delete", "Törlés"),
        ("Properties", "Tulajdonságok"),
        ("Multi Select", "Több fájl kiválasztása"),
        ("Empty Directory", "Üres Könyvtár"),
        ("Not an empty directory", "Nem egy üres könyvtár"),
        ("Are you sure you want to delete this file?", "Biztosan törölni szeretnéd ezt a fájlt?"),
        ("Are you sure you want to delete this empty directory?", "Biztosan törölni szeretnéd ezt az üres könyvtárat?"),
        ("Are you sure you want to delete the file of this directory?", "Biztosan törölni szeretnéd a fájlokat ebben a könyvtárban?"),
        ("Do this for all conflicts", "Ezt tedd az összes konfliktussal"),
        ("This is irreversible!", "Ez a folyamat visszafordíthatatlan!"),
        ("Deleting", "A törlés folyamatban"),
        ("files", "fájlok"),
        ("Waiting", "Várunk"),
        ("Finished", "Végzett"),
        ("Speed", "Gyorsaság"),
        ("Custom Image Quality", "Egyedi Képminőség"),
        ("Privacy mode", "Inkognító mód"),
        ("Block user input", "Felhasználói input blokkokolása"),
        ("Unblock user input", "Felhasználói input blokkolásának feloldása"),
        ("Adjust Window", "Ablakméret beállítása"),
        ("Original", "Eredeti"),
        ("Shrink", "Zsugorított"),
        ("Stretch", "Nyújtott"),
        ("Good image quality", "Jó képminőség"),
        ("Balanced", "Balanszolt"),
        ("Optimize reaction time", "Válaszidő optimializálása"),
        ("Custom", "Egyedi"),
        ("Show remote cursor", "Távoli kurzor mutatása"),
        ("Show quality monitor", "Minőségi monitor mutatása"),
        ("Disable clipboard", "Vágólap Kikapcsolása"),
        ("Lock after session end", "Lezárás a session végén"),
        ("Insert", "Beszúrás"),
        ("Insert Lock", "Beszúrási Zároló"),
        ("Refresh", "Frissítés"),
        ("ID does not exist", "Ez az ID nem létezik"),
        ("Failed to connect to rendezvous server", "A randevú szerverhez való kapcsolódás sikertelen"),
        ("Please try later", "Kérlek próbád később"),
        ("Remote desktop is offline", "A távoli asztal offline"),
        ("Key mismatch", "Eltérés a kulcsokban"),
        ("Timeout", "Időtúllépés"),
        ("Failed to connect to relay server", "A relay szerverhez való kapcsolódás sikertelen"),
        ("Failed to connect via rendezvous server", "A randevú szerverrel való kapcsolódás sikertelen"),
        ("Failed to connect via relay server", "A relay szerverrel való kapcsolódás sikertelen"),
        ("Failed to make direct connection to remote desktop", "A távoli asztalhoz való direkt kapcsolódás sikertelen"),
        ("Set Password", "Jelszó Beállítása"),
        ("OS Password", "Operációs Rendszer Jelszavának Beállítása"),
        ("install_tip", "Az UAC (Felhasználói Fiók Felügyelet) miatt, a RustDesk nem fog rendesen funkcionálni mint távoli oldal néhány esetben. Hogy ezt kikerüld, vagy kikapcsold, kérlek nyomj rá a gombra ezalatt az üzenet alatt, hogy feltelepítsd a RustDesket a rendszerre."),
        ("Click to upgrade", "Kattints a frissítés telepítéséhez"),
        ("Click to download", "Kattints a letöltéshez"),
        ("Click to update", "Kattints a frissítés letöltéséhez"),
        ("Configure", "Beállítás"),
        ("config_acc", "Ahhoz hogy a RustDesket távolról irányítani tudd, \"Elérhetőségi\" jogokat kell adnod a RustDesk-nek."),
        ("config_screen", "Ahhoz hogy a RustDesket távolról irányítani tudd, \"Képernyőfelvételi\" jogokat kell adnod a RustDesk-nek."),
        ("Installing ...", "Telepítés..."),
        ("Install", "Telepítés"),
        ("Installation", "Telepítés"),
        ("Installation Path", "Telepítési útvonal"),
        ("Create start menu shortcuts", "Start menu parancsikon létrehozása"),
        ("Create desktop icon", "Asztali icon létrehozása"),
        ("agreement_tip", "Azzal hogy elindítod a telepítést, elfogadod a licenszszerződést."),
        ("Accept and Install", "Elfogadás és Telepítés"),
        ("End-user license agreement", "Felhasználói licencszerződés"),
        ("Generating ...", "Generálás..."),
        ("Your installation is lower version.", "A jelenleg feltelepített verzió régebbi."),
        ("not_close_tcp_tip", "Ne zárd be ezt az ablakot miközben a tunnelt használod"),
        ("Listening ...", "Halgazózás..."),
        ("Remote Host", "Távoli Host"),
        ("Remote Port", "Távoli Port"),
        ("Action", "Akció"),
        ("Add", "Add"),
        ("Local Port", "Lokális Port"),
        ("setup_server_tip", "Egy gyorsabb kapcsolatért, kérlek hostolj egy saját szervert"),
        ("Too short, at least 6 characters.", "Túl rövid, legalább 6 karakter"),
        ("The confirmation is not identical.", "A megerősítés nem volt azonos"),
        ("Permissions", "Jogok"),
        ("Accept", "Elfogad"),
        ("Dismiss", "Elutasít"),
        ("Disconnect", "Szétkapcsolás"),
        ("Allow using keyboard and mouse", "Billentyűzet és egér használatának engedélyezése"),
        ("Allow using clipboard", "Vágólap használatának engedélyezése"),
        ("Allow hearing sound", "Hang átvitelének engedélyezése"),
        ("Allow file copy and paste", "Fájlok másolásának és beillesztésének engedélyezése"),
        ("Connected", "Kapcsolódva"),
        ("Direct and encrypted connection", "Direkt, és titkosított kapcsolat"),
        ("Relayed and encrypted connection", "Relayelt, és titkosított kapcsolat"),
        ("Direct and unencrypted connection", "Direkt, és nem titkosított kapcsolat"),
        ("Relayed and unencrypted connection", "Rekayelt, és nem titkosított kapcsolat"),
        ("Enter Remote ID", "Kérlek írd be a távoli ID-t"),
        ("Enter your password", "Kérlek írd be a jelszavadat"),
        ("Logging in...", "A belépés folyamatban..."),
        ("Enable RDP session sharing", "Az RDP session megosztás engedélyezése"),
        ("Auto Login", "Automatikus Login"),
        ("Enable Direct IP Access", "Direkt IP elérés engedélyezése"),
        ("Rename", "Átnevezés"),
        ("Space", "Hely"),
        ("Create Desktop Shortcut", "Asztali Parancsikon Lértehozása"),
        ("Change Path", "Útvonal Megváltoztatása"),
        ("Create Folder", "Mappa Készítése"),
        ("Please enter the folder name", "Kérlek írd be a mappa nevét"),
        ("Fix it", "Kérlek javísd meg"),
        ("Warning", "Figyelem"),
        ("Login screen using Wayland is not supported", "A belépési kijelzővel a Wayland használata nem támogatott"),
        ("Reboot required", "Újraindítás szükséges"),
        ("Unsupported display server ", "Nem támogatott kijelző szerver"),
        ("x11 expected", "x11-re számítottt"),
        ("Port", "Port"),
        ("Settings", "Beállítások"),
        ("Username", "Felhasználónév"),
        ("Invalid port", "Érvénytelen port"),
        ("Closed manually by the peer", "A kapcsolat manuálisan be lett zárva a másik fél álltal"),
        ("Enable remote configuration modification", "Távoli konfiguráció módosítás engedélyezése"),
        ("Run without install", "Futtatás feltelepítés nélkül"),
        ("Always connected via relay", "Mindig relay által kapcsolódott"),
        ("Always connect via relay", "Mindig relay által kapcsolódik"),
        ("whitelist_tip", "Csak a fehérlistán lévő címek érhetnek el"),
        ("Login", "Belépés"),
        ("Logout", "Kilépés"),
        ("Tags", "Tagok"),
        ("Search ID", "ID keresés"),
        ("Current Wayland display server is not supported", "Jelenleg a Wayland display szerver nem támogatott"),
        ("whitelist_sep", "Ide jönnek a címek, vesző, pontosvessző, space, vagy új sorral elválasztva"),
        ("Add ID", "ID Hozzáadása"),
        ("Add Tag", "Tag Hozzáadása"),
        ("Unselect all tags", "Az összes tag kiválasztásának törlése"),
        ("Network error", "Hálózati hiba"),
        ("Username missed", "A felhasználónév kimaradt"),
        ("Password missed", "A jelszó kimaradt"),
        ("Wrong credentials", "Hibás felhasználónév vagy jelszó"),
        ("Edit Tag", "A tag(ok) szerkeztése"),
        ("Unremember Password", "A jelszó megjegyzésének törlése"),
        ("Favorites", "Kedvencek"),
        ("Add to Favorites", "Hozzáadás a kedvencekhez"),
        ("Remove from Favorites", "Eltávolítás a kedvencektől"),
        ("Empty", "Üres"),
        ("Invalid folder name", "Helytelen fájlnév"),
        ("Socks5 Proxy", "Socks5-ös Proxy"),
        ("Hostname", "Hostnév"),
        ("Discovered", "Felfedezés"),
        ("install_daemon_tip", "Ahhoz hogy a RustDesk bootkor elinduljon, telepítened kell a rendszer szolgáltatást."),
        ("Remote ID", "Távoli ID"),
        ("Paste", "Beillesztés"),
        ("Paste here?", "Beillesztés ide?"),
        ("Are you sure to close the connection?", "Biztos vagy benne hogy be szeretnéd zárni a kapcsolatot?"),
        ("Download new version", "Új verzó letöltése"),
        ("Touch mode", "Érintési mód bekapcsolása"),
        ("Mouse mode", "Egérhasználati mód bekapcsolása"),
        ("One-Finger Tap", "Egyújas érintés"),
        ("Left Mouse", "Baloldali Egér"),
        ("One-Long Tap", "Egy hosszú érintés"),
        ("Two-Finger Tap", "Két újas érintés"),
        ("Right Mouse", "Jobboldali Egér"),
        ("One-Finger Move", "Egyújas mozgatás"),
        ("Double Tap & Move", "Kétszeri érintés, és Mozgatás"),
        ("Mouse Drag", "Egérrel való húzás"),
        ("Three-Finger vertically", "Három ujj függőlegesen"),
        ("Mouse Wheel", "Egérgörgő"),
        ("Two-Finger Move", "Kátújas mozgatás"),
        ("Canvas Move", "Nézet Mozgatása"),
        ("Pinch to Zoom", "Húzd össze a nagyításhoz"),
        ("Canvas Zoom", "Nézet Nagyítása"),
        ("Reset canvas", "Nézet visszaállítása"),
        ("No permission of file transfer", "Nincs jogod fájl transzer indításához"),
        ("Note", "Megyjegyzés"),
        ("Connection", "Kapcsolat"),
        ("Share Screen", "Képernyőmegosztás"),
        ("CLOSE", "LETILT"),
        ("OPEN", "ENGEDÉLYEZ"),
        ("Chat", "Chat"),
        ("Total", "Összes"),
        ("items", "Tárgyak"),
        ("Selected", "Kiválasztott"),
        ("Screen Capture", "Képernyőrögzítés"),
        ("Input Control", "Input Kontrol"),
        ("Audio Capture", "Audió Rögzítés"),
        ("File Connection", "Fájlkapcsolat"),
        ("Screen Connection", "Új Vizuális Kapcsolat"),
        ("Do you accept?", "Elfogadod?"),
        ("Open System Setting", "Rendszer beállítások megnyitása"),
        ("How to get Android input permission?", "Hogyan állíthatok be Android input jogokat?"),
        ("android_input_permission_tip1", "Ahhoz hogy egy távoli eszköz kontolálhassa az Android eszközödet egérrel vagy érintéssel, jogot kell adnod a RustDesk-nek, hogy használja az \"Elérhetőségi\" szolgáltatást."),
        ("android_input_permission_tip2", "Kérlek navigálj a rendszer beállításaihoz, keresd meg vagy írd be hogy [Feltelepített Szolgáltatások], és kapcsold be a [RustDesk Input] szolgáltatást."),
        ("android_new_connection_tip", "Új kontrollálási kérés érkezett, amely irányítani szeretné az eszközöded."),
        ("android_service_will_start_tip", "A \"Képernyőrögzítés\" engedélyezése automatikusan elindítja majd a szolgáltatást, amely megengedi más eszközöknek hogy kérést kezdeményezzenek az eszköz felé."),
        ("android_stop_service_tip", "A szolgáltatás bezárása automatikusan szétkapcsol minden létező kapcsolatot."),
        ("android_version_audio_tip", "A jelenlegi Android verzió nem támogatja a hangrögzítést, kérlek frissíts legalább Android 10-re, vagy egy újabb verzióra."),
        ("android_start_service_tip", "Nyomj a [Szolgáltatás Indítása] opcióra, vagy adj [Képernyőrözítési] jogot az applikációnak hogy elindítsd a képernyőmegosztó szolgáltatást."),
        ("Account", "Fiók"),
        ("Overwrite", "Felülírás"),
        ("This file exists, skip or overwrite this file?", "Ez a fájl már létezik, skippeljünk, vagy felülírjuk ezt a fájlt?"),
        ("Quit", "Kilépés"),
        ("doc_mac_permission", "https://rustdesk.com/docs/hu/manual/mac/#enable-permissions"),
        ("Help", "Segítség"),
        ("Failed", "Sikertelen"),
        ("Succeeded", "Sikeres"),
        ("Someone turns on privacy mode, exit", "Valaki bekacsolta a privát módot, lépj ki"),
        ("Unsupported", "Nem támogatott"),
        ("Peer denied", "Elutasítva a távoli fél álltal"),
        ("Please install plugins", "Kérlek telepítsd a pluginokat"),
        ("Peer exit", "A távoli fél kilépett"),
        ("Failed to turn off", "Nem tudtuk kikapcsolni"),
        ("Turned off", "Kikapcsolva"),
        ("In privacy mode", "Belépés a privát módba"),
        ("Out privacy mode", "Kilépés a privát módból"),
        ("Language", "Nyelv"),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", ""),
        ("Use temporary password", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Set temporary password length", ""),
        ("Enable Remote Restart", ""),
        ("Allow remote restart", ""),
        ("Restart Remote Device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting Remote Device", ""),
        ("Remote device is restarting, please close this message box and reconnect with permanent password after a while", ""),
    ].iter().cloned().collect();
}
