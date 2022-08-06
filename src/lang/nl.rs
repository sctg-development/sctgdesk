lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Uw Bureaublad"),
        ("desk_tip", "Uw bureaublad is toegankelijk via de ID en het wachtwoord hieronder."),
        ("Password", "Wachtwoord"),
        ("Ready", "Klaar"),
        ("Established", "Bevestigd"),
        ("connecting_status", "Verbinding maken met het RustDesk netwerk..."),
        ("Enable Service", "Service Inschakelen"),
        ("Start Service", "Start Service"),
        ("Service is running", "De service draait."),
        ("Service is not running", "De service draait niet"),
        ("not_ready_status", "Niet klaar, controleer de netwerkverbinding"),
        ("Control Remote Desktop", "Beheer Extern Bureaublad"),
        ("Transfer File", "Bestand Overzetten"),
        ("Connect", "Verbinden"),
        ("Recent Sessions", "Recente Behandelingen"),
        ("Address Book", "Adresboek"),
        ("Confirmation", "Bevestiging"),
        ("TCP Tunneling", "TCP Tunneling"),
        ("Remove", "Verwijder"),
        ("Refresh random password", "Vernieuw willekeurig wachtwoord"),
        ("Set your own password", "Stel je eigen wachtwoord in"),
        ("Enable Keyboard/Mouse", "Toetsenbord/Muis Inschakelen"),
        ("Enable Clipboard", "Klembord Inschakelen"),
        ("Enable File Transfer", "Bestandsoverdracht Inschakelen"),
        ("Enable TCP Tunneling", "TCP Tunneling Inschakelen"),
        ("IP Whitelisting", "IP Witte Lijst"),
        ("ID/Relay Server", "ID/Relay Server"),
        ("Stop service", "Stop service"),
        ("Change ID", "Wijzig ID"),
        ("Website", "Website"),
        ("About", "Over"),
        ("Mute", "Geluid uit"),
        ("Audio Input", "Audio Ingang"),
        ("Enhancements", "Verbeteringen"),
        ("Hardware Codec", "Hardware Codec"),
        ("Adaptive Bitrate", "Aangepaste Bitsnelheid"),
        ("ID Server", "ID Server"),
        ("Relay Server", "Relay Server"),
        ("API Server", "API Server"),
        ("invalid_http", "Moet beginnen met http:// of https://"),
        ("Invalid IP", "Ongeldig IP"),
        ("id_change_tip", "Alleen de letters a-z, A-Z, 0-9, _ (underscore) kunnen worden gebruikt. De eerste letter moet a-z, A-Z zijn. De lengte moet tussen 6 en 16 liggen."),
        ("Invalid format", "Ongeldig formaat"),
        ("server_not_support", "Nog niet ondersteund door de server"),
        ("Not available", "Niet beschikbaar"),
        ("Too frequent", "Te vaak"),
        ("Cancel", "Annuleer"),
        ("Skip", "Overslaan"),
        ("Close", "Sluit"),
        ("Retry", "Probeer opnieuw"),
        ("OK", "OK"),
        ("Password Required", "Wachtwoord vereist"),
        ("Please enter your password", "Voer uw wachtwoord in"),
        ("Remember password", "Wachtwoord onthouden"),
        ("Wrong Password", "Verkeerd wachtwoord"),
        ("Do you want to enter again?", "Wil je opnieuw invoeren?"),
        ("Connection Error", "Fout bij verbinding"),
        ("Error", "Fout"),
        ("Reset by the peer", "Reset door de peer"),
        ("Connecting...", "Verbinding maken..."),
        ("Connection in progress. Please wait.", "Verbinding in uitvoering. Even geduld a.u.b."),
        ("Please try 1 minute later", "Probeer 1 minuut later"),
        ("Login Error", "Login Fout"),
        ("Successful", "Succesvol"),
        ("Connected, waiting for image...", "Verbonden, wacht op beeld..."),
        ("Name", "Naam"),
        ("Type", "Type"),
        ("Modified", "Gewijzigd"),
        ("Size", "Grootte"),
        ("Show Hidden Files", "Verborgen bestanden tonen"),
        ("Receive", "Ontvangen"),
        ("Send", "Verzenden"),
        ("Refresh File", "Bestand Verversen"),
        ("Local", "Lokaal"),
        ("Remote", "Op afstand"),
        ("Remote Computer", "Externe Computer"),
        ("Local Computer", "Locale Computer"),
        ("Confirm Delete", "Bevestig Verwijderen"),
        ("Delete", "Verwijder"),
        ("Properties", "Eigenschappen"),
        ("Multi Select", "Meervoudig selecteren"),
        ("Empty Directory", "Lege Map"),
        ("Not an empty directory", "Geen Lege Map"),
        ("Are you sure you want to delete this file?", "Weet je zeker dat je dit bestand wilt verwijderen?"),
        ("Are you sure you want to delete this empty directory?", "Weet je zeker dat je deze lege map wilt verwijderen?"),
        ("Are you sure you want to delete the file of this directory?", "Weet je zeker dat je het bestand uit deze map wilt verwijderen?"),
        ("Do this for all conflicts", "Doe dit voor alle conflicten"),
        ("This is irreversible!", "Dit is onomkeerbaar!"),
        ("Deleting", "Verwijderen"),
        ("files", "bestanden"),
        ("Waiting", "Wachten"),
        ("Finished", "Voltooid"),
        ("Speed", "Snelheid"),
        ("Custom Image Quality", "Aangepaste beeldkwaliteit"),
        ("Privacy mode", "Privacymodus"),
        ("Block user input", "Gebruikersinvoer blokkeren"),
        ("Unblock user input", "Gebruikersinvoer opheffen"),
        ("Adjust Window", "Venster Aanpassen"),
        ("Original", "Origineel"),
        ("Shrink", "Verkleinen"),
        ("Stretch", "Uitrekken"),
        ("Good image quality", "Goede beeldkwaliteit"),
        ("Balanced", "Gebalanceerd"),
        ("Optimize reaction time", "Optimaliseer reactietijd"),
        ("Custom", "Aangepast"),
        ("Show remote cursor", "Toon cursor op extern bureaublad"),
        ("Show quality monitor", "Kwaliteitsmonitor tonen"),
        ("Disable clipboard", "Klembord uitschakelen"),
        ("Lock after session end", "Vergrendelen na einde sessie"),
        ("Insert", "Invoegen"),
        ("Insert Lock", "Vergrendeling Invoegen"),
        ("Refresh", "Vernieuwen"),
        ("ID does not exist", "ID bestaat niet"),
        ("Failed to connect to rendezvous server", "Verbinding met rendez-vous-server mislukt"),
        ("Please try later", "Probeer later opnieuw"),
        ("Remote desktop is offline", "Extern bureaublad is offline"),
        ("Key mismatch", "Code onjuist"),
        ("Timeout", "Time-out"),
        ("Failed to connect to relay server", "Verbinding met relayserver mislukt"),
        ("Failed to connect via rendezvous server", "Verbinding via rendez-vous-server mislukt"),
        ("Failed to connect via relay server", "Verbinding via relaisserver mislukt"),
        ("Failed to make direct connection to remote desktop", "Onmogelijk direct verbinding te maken met extern bureaublad"),
        ("Set Password", "Wachtwoord Instellen"),
        ("OS Password", "OS Wachtwoord"),
        ("install_tip", "Je gebruikt een niet ge�nstalleerde versie. Als gevolg van UAC-beperkingen is het in sommige gevallen niet mogelijk om als controleterminal de muis en het toetsenbord te bedienen of het scherm over te nemen. Klik op de knop hieronder om RustDesk op het systeem te installeren om het bovenstaande probleem te voorkomen."),
        ("Click to upgrade", "Klik voor upgrade"),
        ("Click to download", "Klik om te downloaden"),
        ("Click to update", "Klik om bij te werken"),
        ("Configure", "Configureren"),
        ("config_acc", "Om je bureaublad op afstand te kunnen bedienen, moet je RustDesk \"toegankelijkheid\" toestemming geven."),
        ("config_screen", "Om toegang te krijgen tot het externe bureaublad, moet je RustDesk de toestemming \"schermregistratie\" geven."),
        ("Installing ...", "Installeren ..."),
        ("Install", "Installeer"),
        ("Installation", "Installatie"),
        ("Installation Path", "Installatie Pad"),
        ("Create start menu shortcuts", "Startmenu snelkoppelingen maken"),
        ("Create desktop icon", "Bureaubladpictogram maken"),
        ("agreement_tip", "Het starten van de installatie betekent het accepteren van de licentieovereenkomst."),
        ("Accept and Install", "Accepteren en installeren"),
        ("End-user license agreement", "Licentieovereenkomst eindgebruiker"),
        ("Generating ...", "Genereert ..."),
        ("Your installation is lower version.", "Uw installatie is een lagere versie."),
        ("not_close_tcp_tip", "Gelieve dit venster niet te sluiten wanneer u de tunnel gebruikt"),
        ("Listening ...", "Luisteren ..."),
        ("Remote Host", "Externe Host"),
        ("Remote Port", "Externe Poort"),
        ("Action", "Actie"),
        ("Add", "Toevoegen"),
        ("Local Port", "Lokale Poort"),
        ("setup_server_tip", "Als u een snellere verbindingssnelheid nodig heeft, kunt u ervoor kiezen om uw eigen server te cre�ren"),
        ("Too short, at least 6 characters.", "Te kort, minstens 6 tekens."),
        ("The confirmation is not identical.", "De bevestiging is niet identiek."),
        ("Permissions", "Machtigingen"),
        ("Accept", "Accepteren"),
        ("Dismiss", "Afwijzen"),
        ("Disconnect", "Verbinding verbreken"),
        ("Allow using keyboard and mouse", "Gebruik toetsenbord en muis toestaan"),
        ("Allow using clipboard", "Gebruik klembord toestaan"),
        ("Allow hearing sound", "Geluidsweergave toestaan"),
        ("Allow file copy and paste", "Kopi�ren en plakken van bestanden toestaan"),
        ("Connected", "Verbonden"),
        ("Direct and encrypted connection", "Directe en versleutelde verbinding"),
        ("Relayed and encrypted connection", "Doorgeschakelde en versleutelde verbinding"),
        ("Direct and unencrypted connection", "Directe en niet-versleutelde verbinding"),
        ("Relayed and unencrypted connection", "Doorgeschakelde en niet-versleutelde verbinding"),
        ("Enter Remote ID", "Voer Extern ID in"),
        ("Enter your password", "Voer uw wachtwoord in"),
        ("Logging in...", "Aanmelden..."),
        ("Enable RDP session sharing", "Delen van RDP-sessie inschakelen"),
        ("Auto Login", "Automatisch Aanmelden"),
        ("Enable Direct IP Access", "Directe IP-toegang Inschakelen"),
        ("Rename", "Naam wijzigen"),
        ("Space", "Spatie"),
        ("Create Desktop Shortcut", "Snelkoppeling op bureaublad maken"),
        ("Change Path", "Pad wijzigen"),
        ("Create Folder", "Map Maken"),
        ("Please enter the folder name", "Geef de mapnaam op"),
        ("Fix it", "Repareer het"),
        ("Warning", "Waarschuwing"),
        ("Login screen using Wayland is not supported", "Aanmeldingsscherm via Wayland wordt niet ondersteund"),
        ("Reboot required", "Opnieuw opstarten vereist"),
        ("Unsupported display server ", "Niet-ondersteunde weergaveserver"),
        ("x11 expected", "x11 verwacht"),
        ("Port", "P�ort"),
        ("Settings", "Instellingen"),
        ("Username", "Gebruikersnaam"),
        ("Invalid port", "Ongeldige poort"),
        ("Closed manually by the peer", "Handmatig gesloten door de peer"),
        ("Enable remote configuration modification", "Wijziging configuratie op afstand inschakelen"),
        ("Run without install", "Uitvoeren zonder installatie"),
        ("Always connected via relay", "Altijd verbonden via relay"),
        ("Always connect via relay", "Altijd verbinden via relay"),
        ("whitelist_tip", "Alleen een IP-adres op de witte lijst krijgt toegang tot mijn toestel"),
        ("Login", "Log In"),
        ("Logout", "Log Uit"),
        ("Tags", "Labels"),
        ("Search ID", "Zoek ID"),
        ("Current Wayland display server is not supported", "Huidige Wayland weergaveserver wordt niet ondersteund"),
        ("whitelist_sep", "Gescheiden door komma, puntkomma, spatie of nieuwe regel"),
        ("Add ID", "ID Toevoegen"),
        ("Add Tag", "Label Toevoegen"),
        ("Unselect all tags", "Alle labels verwijderen"),
        ("Network error", "Netwerkfout"),
        ("Username missed", "Gebruikersnaam gemist"),
        ("Password missed", "Wachtwoord vergeten"),
        ("Wrong credentials", "Verkeerde inloggegevens"),
        ("Edit Tag", "Label Bewerken"),
        ("Unremember Password", "Wachtwoord vergeten"),
        ("Favorites", "Favorieten"),
        ("Add to Favorites", "Toevoegen aan Favorieten"),
        ("Remove from Favorites", "Verwijderen uit Favorieten"),
        ("Empty", "Leeg"),
        ("Invalid folder name", "Ongeldige mapnaam"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Hostnaam"),
        ("Discovered", "Ontdekt"),
        ("install_daemon_tip", "Om bij het opstarten van de computer te kunnen beginnen, moet je de systeemdienst installeren."),
        ("Remote ID", "Externe ID"),
        ("Paste", "Plakken"),
        ("Paste here?", "Hier plakken"),
        ("Are you sure to close the connection?", "Weet je zeker dat je de verbinding wilt sluiten?"),
        ("Download new version", "Download nieuwe versie"),
        ("Touch mode", "Aanraak modus"),
        ("Mouse mode", "Muismodus"),
        ("One-Finger Tap", "E�n-Vinger Tik"),
        ("Left Mouse", "Linkermuis"),
        ("One-Long Tap", "��n-Vinger-Lange-Tik"),
        ("Two-Finger Tap", "Twee-Vingers-Tik"),
        ("Right Mouse", "Rechter muis"),
        ("One-Finger Move", "E�n-Vinger-Verplaatsing"),
        ("Double Tap & Move", "Dubbel Tik en Verplaatsen"),
        ("Mouse Drag", "Muis Slepen"),
        ("Three-Finger vertically", "Drie-Vinger verticaal"),
        ("Mouse Wheel", "Muiswiel"),
        ("Two-Finger Move", "Twee-Vingers Verplaatsen"),
        ("Canvas Move", "Canvas Verplaatsen"),
        ("Pinch to Zoom", "Knijp om te Zoomen"),
        ("Canvas Zoom", "Canvas Zoom"),
        ("Reset canvas", "Reset canvas"),
        ("No permission of file transfer", "Geen toestemming voor bestandsoverdracht"),
        ("Note", "Opmerking"),
        ("Connection", "Verbinding"),
        ("Share Screen", "Scherm Delen"),
        ("CLOSE", "SLUITEN"),
        ("OPEN", "OPEN"),
        ("Chat", "Chat"),
        ("Total", "Totaal"),
        ("items", "items"),
        ("Selected", "Geselecteerd"),
        ("Screen Capture", "Schermopname"),
        ("Input Control", "Invoercontrole"),
        ("Audio Capture", "Audio Opnemen"),
        ("File Connection", "Bestandsverbinding"),
        ("Screen Connection", "Schermverbinding"),
        ("Do you accept?", "Sta je toe?"),
        ("Open System Setting", "Systeeminstelling Openen"),
        ("How to get Android input permission?", "Hoe krijg ik Android invoer toestemming?"),
        ("android_input_permission_tip1", "Om ervoor te zorgen dat een extern apparaat uw Android-apparaat kan besturen via muis of aanraking, moet u RustDesk toestaan om de \"Toegankelijkheid\" service te gebruiken."),
        ("android_input_permission_tip2", "Ga naar de volgende pagina met systeeminstellingen, zoek en ga naar [Ge�nstalleerde Services], schakel de service [RustDesk Input] in."),
        ("android_new_connection_tip", "Er is een nieuw controleverzoek binnengekomen, dat uw huidige apparaat wil controleren."),
        ("android_service_will_start_tip", "Als u \"Schermopname\" inschakelt, wordt de service automatisch gestart, zodat andere apparaten een verbinding met uw apparaat kunnen aanvragen."),
        ("android_stop_service_tip", "Het sluiten van de service zal automatisch alle gemaakte verbindingen sluiten."),
        ("android_version_audio_tip", "De huidige versie van Android ondersteunt geen audio-opname, upgrade naar Android 10 of hoger."),
        ("android_start_service_tip", "Druk op [Start Service] of op de permissie OPEN [Screenshot] om de service voor het overnemen van het scherm te starten."),
        ("Account", "Account"),
        ("Overwrite", "Overschrijven"),
        ("This file exists, skip or overwrite this file?", "Dit bestand bestaat reeds,  overslaan of overschrijven?"),
        ("Quit", "Afsluiten"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "https://rustdesk.com/docs/en/manual/linux/#x11-required"),
        ("Failed", "Mislukt"),
        ("Succeeded", "Geslaagd"),
        ("Someone turns on privacy mode, exit", "Iemand schakelt privacymodus in, afsluiten"),
        ("Unsupported", "Niet Ondersteund"),
        ("Peer denied", "Peer geweigerd"),
        ("Please install plugins", "Installeer plugins"),
        ("Peer exit", "Peer afgesloten"),
        ("Failed to turn off", "Uitschakelen mislukt"),
        ("Turned off", "Uitgeschakeld"),
        ("In privacy mode", "In privacymodus"),
        ("Out privacy mode", "Uit privacymodus"),
        ("Language", "Taal"),
        ("Keep RustDesk background service", "RustDesk achtergronddienst behouden"),
        ("Ignore Battery Optimizations", "Negeer Batterij Optimalisaties"),
        ("android_open_battery_optimizations_tip", "Ga naar de volgende pagina met instellingen"),
        ("Connection not allowed", "Verbinding niet toegestaan"),
        ("Use temporary password", "Tijdelijk wachtwoord gebruiken"),
        ("Use permanent password", "Gebruik permanent wachtwoord"),
        ("Use both passwords", "Gebruik beide wachtwoorden"),
        ("Set permanent password", "Stel permanent wachtwoord in"),
        ("Set temporary password length", "Lengte tijdelijk wachtwoord instellen"),
        ("Enable Remote Restart", "Schakel Herstart op afstand in"),
        ("Allow remote restart", "Opnieuw Opstarten op afstand toestaan"),
        ("Restart Remote Device", "Apparaat op afstand herstarten"),
        ("Are you sure you want to restart", "Weet je zeker dat je wilt herstarten"),
        ("Restarting Remote Device", "Apparaat op afstand herstarten"),
        ("remote_restarting_tip", "Apparaat op afstand wordt opnieuw opgestart, sluit dit bericht en maak na een ogenblik opnieuw verbinding met het permanente wachtwoord."),
    ].iter().cloned().collect();
}
