lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Uw Bureaublad"),
        ("desk_tip", "Uw bureaublad is toegankelijk via de ID en het wachtwoord hieronder."),
        ("Password", "Wachtwoord"),
        ("Ready", "Klaar"),
        ("Established", "Opgezet"),
        ("connecting_status", "Verbinding maken met het RustDesk netwerk..."),
        ("Enable Service", "Service Inschakelen"),
        ("Start Service", "Start Service"),
        ("Service is running", "De service loopt."),
        ("Service is not running", "De service loopt niet"),
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
        ("Import Server Config", "Importeer Serverconfiguratie"),
        ("Export Server Config", "Exporteer Serverconfiguratie"),
        ("Import server configuration successfully", "Importeren serverconfiguratie succesvol"),
        ("Export server configuration successfully", "Exporteren serverconfiguratie succesvol"),
        ("Invalid server configuration", "Ongeldige Serverconfiguratie"),
        ("Clipboard is empty", "Klembord is leeg"),
        ("Stop service", "Stop service"),
        ("Change ID", "Wijzig ID"),
        ("Your new ID", "Uw nieuw ID"),
        ("length %min% to %max%", "lengte %min% tot %max%"),
        ("starts with a letter", "begint met een letter"),
        ("allowed characters", "toegestane tekens"),
        ("id_change_tip", "Alleen de letters a-z, A-Z, 0-9, _ (underscore) kunnen worden gebruikt. De eerste letter moet a-z, A-Z zijn. De lengte moet tussen 6 en 16 liggen."),
        ("Website", "Website"),
        ("About", "Over"),
        ("Slogan_tip", "Ontwikkeld met het hart voor deze chaotische wereld!"),
        ("Privacy Statement", "Privacyverklaring"),
        ("Mute", "Geluid uit"),
        ("Build Date", "Versie datum"),
        ("Version", "Versie"),
        ("Home", "Startpagina"),
        ("Audio Input", "Audio Ingang"),
        ("Enhancements", "Verbeteringen"),
        ("Hardware Codec", "Hardware Codec"),
        ("Adaptive bitrate", "Aangepaste Bitsnelheid"),
        ("ID Server", "Server ID"),
        ("Relay Server", "Relay Server"),
        ("API Server", "API Server"),
        ("invalid_http", "Moet beginnen met http:// of https://"),
        ("Invalid IP", "Ongeldig IP"),
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
        ("Please enter your password", "Geef uw wachtwoord in"),
        ("Remember password", "Wachtwoord onthouden"),
        ("Wrong Password", "Verkeerd wachtwoord"),
        ("Do you want to enter again?", "Wil je opnieuw ingeven?"),
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
        ("Show Hidden Files", "Toon verborgen bestanden"),
        ("Receive", "Ontvangen"),
        ("Send", "Verzenden"),
        ("Refresh File", "Bestand Verversen"),
        ("Local", "Lokaal"),
        ("Remote", "Op afstand"),
        ("Remote Computer", "Externe Computer"),
        ("Local Computer", "Lokale Computer"),
        ("Confirm Delete", "Bevestig Verwijderen"),
        ("Delete", "Verwijder"),
        ("Properties", "Eigenschappen"),
        ("Multi Select", "Meervoudig selecteren"),
        ("Select All", "Selecteer Alle"),
        ("Unselect All", "Deselecteer alles"),
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
        ("Scrollbar", "Schuifbalk"),
        ("ScrollAuto", "Auto Schuiven"),
        ("Good image quality", "Goede beeldkwaliteit"),
        ("Balanced", "Gebalanceerd"),
        ("Optimize reaction time", "Optimaliseer reactietijd"),
        ("Custom", "Aangepast"),
        ("Show remote cursor", "Toon cursor van extern bureaublad"),
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
        ("install_tip", "Je gebruikt een niet geinstalleerde versie. Als gevolg van UAC-beperkingen is het in sommige gevallen niet mogelijk om als controleterminal de muis en het toetsenbord te bedienen of het scherm over te nemen. Klik op de knop hieronder om RustDesk op het systeem te installeren om het bovenstaande probleem te voorkomen."),
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
        ("Local Address", "Lokaal Adres"),
        ("Change Local Port", "Wijzig Lokale Poort"),
        ("setup_server_tip", "Als u een snellere verbindingssnelheid nodig heeft, kunt u ervoor kiezen om uw eigen server aan te maken"),
        ("Too short, at least 6 characters.", "e kort, minstens 6 tekens."),
        ("The confirmation is not identical.", "De bevestiging is niet identiek."),
        ("Permissions", "Machtigingen"),
        ("Accept", "Accepteren"),
        ("Dismiss", "Afwijzen"),
        ("Disconnect", "Verbinding verbreken"),
        ("Allow using keyboard and mouse", "Gebruik toetsenbord en muis toestaan"),
        ("Allow using clipboard", "Gebruik klembord toestaan"),
        ("Allow hearing sound", "Geluidsweergave toestaan"),
        ("Allow file copy and paste", "Kopieren en plakken van bestanden toestaan"),
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
        ("Enable Direct IP Access", "Directe IP-toegang inschakelen"),
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
        ("Unsupported display server", "Niet-ondersteunde weergaveserver"),
        ("x11 expected", "x11 verwacht"),
        ("Port", "Poort"),
        ("Settings", "Instellingen"),
        ("Username", "Gebruikersnaam"),
        ("Invalid port", "Ongeldige poort"),
        ("Closed manually by the peer", "Handmatig gesloten door de peer"),
        ("Enable remote configuration modification", "Wijziging configuratie op afstand inschakelen"),
        ("Run without install", "Uitvoeren zonder installatie"),
        ("Connect via relay", "Verbinden via relais"),
        ("Always connect via relay", "Altijd verbinden via relay"),
        ("whitelist_tip", "Alleen een IP-adres op de witte lijst krijgt toegang tot mijn toestel"),
        ("Login", "Log In"),
        ("Verify", "Controleer"),
        ("Remember me", "Herinner mij"),
        ("Trust this device", "Vertrouw dit apparaat"),
        ("Verification code", "Verificatie code"),
        ("verification_tip", "Er is een nieuw apparaat gedetecteerd en er is een verificatiecode naar het geregistreerde e-mailadres gestuurd, voer de verificatiecode in om de verbinding voort te zetten."),
        ("Logout", "Log Uit"),
        ("Tags", "Labels"),
        ("Search ID", "Zoek ID"),
        ("whitelist_sep", "Gescheiden door komma, puntkomma, spatie of nieuwe regel"),
        ("Add ID", "ID Toevoegen"),
        ("Add Tag", "Label Toevoegen"),
        ("Unselect all tags", "Alle labels verwijderen"),
        ("Network error", "Netwerkfout"),
        ("Username missed", "Gebruikersnaam gemist"),
        ("Password missed", "Wachtwoord vergeten"),
        ("Wrong credentials", "Verkeerde inloggegevens"),
        ("The verification code is incorrect or has expired", ""),
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
        ("One-Finger Tap", "Een-Vinger Tik"),
        ("Left Mouse", "Linkermuis"),
        ("One-Long Tap", "Een-Vinger-Lange-Tik"),
        ("Two-Finger Tap", "Twee-Vingers-Tik"),
        ("Right Mouse", "Rechter muis"),
        ("One-Finger Move", "Een-Vinger-Verplaatsing"),
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
        ("android_input_permission_tip2", "Ga naar de volgende pagina met systeeminstellingen, zoek en ga naar [Geinstalleerde Services], schakel de service [RustDesk Input] in."),
        ("android_new_connection_tip", "Er is een nieuw controleverzoek binnengekomen, dat uw huidige apparaat wil controleren."),
        ("android_service_will_start_tip", "Als u \"Schermopname\" inschakelt, wordt de service automatisch gestart, zodat andere apparaten een verbinding met uw apparaat kunnen aanvragen."),
        ("android_stop_service_tip", "Het sluiten van de service zal automatisch alle gemaakte verbindingen sluiten."),
        ("android_version_audio_tip", "De huidige versie van Android ondersteunt geen audio-opname, upgrade naar Android 10 of hoger."),
        ("android_start_service_tip", "Druk op [Start service] of activeer de autorisatie [Scherm opnemen] om de schermdelingsservice te starten."),
        ("android_permission_may_not_change_tip", "Toestemmingen voor tot stand gebrachte verbindingen kunnen niet onmiddellijk worden gewijzigd totdat er opnieuw verbinding wordt gemaakt."),
        ("Account", "Account"),
        ("Overwrite", "Overschrijven"),
        ("This file exists, skip or overwrite this file?", "Dit bestand bestaat reeds, overslaan of overschrijven?"),
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
        ("Start on Boot", "Starten bij Opstarten"),
        ("Start the screen sharing service on boot, requires special permissions", "Start de schermdelings service bij het opstarten, vereist speciale rechten"),
        ("Connection not allowed", "Verbinding niet toegestaan"),
        ("Legacy mode", "Verouderde modus"),
        ("Map mode", "Map mode"),
        ("Translate mode", "Vertaalmodus"),
        ("Use permanent password", "Gebruik permanent wachtwoord"),
        ("Use both passwords", "Gebruik beide wachtwoorden"),
        ("Set permanent password", "Stel permanent wachtwoord in"),
        ("Enable Remote Restart", "Schakel Herstart op afstand in"),
        ("Allow remote restart", "Opnieuw Opstarten op afstand toestaan"),
        ("Restart Remote Device", "Apparaat op afstand herstarten"),
        ("Are you sure you want to restart", "Weet je zeker dat je wilt herstarten"),
        ("Restarting Remote Device", "Apparaat op afstand herstarten"),
        ("remote_restarting_tip", "Apparaat op afstand wordt opnieuw opgestart, sluit dit bericht en maak na een ogenblik opnieuw verbinding met het permanente wachtwoord."),
        ("Copied", "Gekopieerd"),
        ("Exit Fullscreen", "Volledig Scherm sluiten"),
        ("Fullscreen", "Volledig Scherm"),
        ("Mobile Actions", "Mobiele Acties"),
        ("Select Monitor", "Selecteer Monitor"),
        ("Control Actions", "Controleacties"),
        ("Display Settings", "Beeldscherminstellingen"),
        ("Ratio", "Verhouding"),
        ("Image Quality", "Beeldkwaliteit"),
        ("Scroll Style", "Scroll Stijl"),
        ("Show Toolbar", "Werkbalk Weergeven"),
        ("Hide Toolbar", "Verberg Werkbalk"),
        ("Direct Connection", "Directe Verbinding"),
        ("Relay Connection", "Relaisverbinding"),
        ("Secure Connection", "Beveiligde Verbinding"),
        ("Insecure Connection", "Onveilige Verbinding"),
        ("Scale original", "Oorspronkelijke schaal"),
        ("Scale adaptive", "Schaalaanpassing"),
        ("General", "Algemeen"),
        ("Security", "Beveiliging"),
        ("Theme", "Thema"),
        ("Dark Theme", "Donker Thema"),
        ("Light Theme", "Lichte Thema"),
        ("Dark", "Donker"),
        ("Light", "Licht"),
        ("Follow System", "Volg Systeem"),
        ("Enable hardware codec", "Hardware codec inschakelen"),
        ("Unlock Security Settings", "Beveiligingsinstellingen vrijgeven"),
        ("Enable Audio", "Audio Inschakelen"),
        ("Unlock Network Settings", "Netwerkinstellingen Vrijgeven"),
        ("Server", "Server"),
        ("Direct IP Access", "Directe IP toegang"),
        ("Proxy", "Proxy"),
        ("Apply", "Toepassen"),
        ("Disconnect all devices?", "Alle apparaten uitschakelen?"),
        ("Clear", "Wis"),
        ("Audio Input Device", "Audio-invoerapparaat"),
        ("Use IP Whitelisting", "Gebruik een witte lijst van IP-adressen"),
        ("Network", "Netwerk"),
        ("Enable RDP", "Zet RDP aan"),
        ("Pin Toolbar", "Werkbalk Vastzetten"),
        ("Unpin Toolbar", "Werkbalk Losmaken"),
        ("Recording", "Opnemen"),
        ("Directory", "Map"),
        ("Automatically record incoming sessions", "Automatisch inkomende sessies opnemen"),
        ("Change", "Wissel"),
        ("Start session recording", "Start de sessieopname"),
        ("Stop session recording", "Stop de sessieopname"),
        ("Enable Recording Session", "Opnamesessie Activeren"),
        ("Allow recording session", "Opnamesessie toestaan"),
        ("Enable LAN Discovery", "LAN-detectie inschakelen"),
        ("Deny LAN Discovery", "LAN-detectie Weigeren"),
        ("Write a message", "Schrijf een bericht"),
        ("Prompt", "Verzoek"),
        ("Please wait for confirmation of UAC...", "Wacht op bevestiging van UAC..."),
        ("elevated_foreground_window_tip", "Het momenteel geopende venster van de op afstand bediende computer vereist hogere rechten. Daarom is het momenteel niet mogelijk de muis en het toetsenbord te gebruiken. Vraag de gebruiker wiens computer u op afstand bedient om het venster te minimaliseren of de rechten te verhogen. Om dit probleem in de toekomst te voorkomen, wordt aanbevolen de software te installeren op de op afstand bediende computer."),
        ("Disconnected", "Afgesloten"),
        ("Other", "Andere"),
        ("Confirm before closing multiple tabs", "Bevestig voordat u meerdere tabbladen sluit"),
        ("Keyboard Settings", "Toetsenbord instellingen"),
        ("Full Access", "Volledige Toegang"),
        ("Screen Share", "Scherm Delen"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland vereist Ubuntu 21.04 of een hogere versie."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland vereist een hogere versie van Linux distro. Probeer X11 desktop of verander je OS."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Selecteer het scherm dat moet worden gedeeld (Bediening aan de kant van de peer)."),
        ("Show RustDesk", "Toon RustDesk"),
        ("This PC", "Deze PC"),
        ("or", "of"),
        ("Continue with", "Ga verder met"),
        ("Elevate", "Verhoog"),
        ("Zoom cursor", "Cursor Zoomen"),
        ("Accept sessions via password", "Sessies accepteren via wachtwoord"),
        ("Accept sessions via click", "Sessies accepteren via klik"),
        ("Accept sessions via both", "Accepteer sessies via beide"),
        ("Please wait for the remote side to accept your session request...", "Wacht tot de andere kant uw sessieverzoek accepteert..."),
        ("One-time Password", "Eenmalig Wachtwoord"),
        ("Use one-time password", "Gebruik een eenmalig Wachtwoord"),
        ("One-time password length", "Eenmalig Wachtwoord lengre"),
        ("Request access to your device", "Toegang tot uw toestel aanvragen"),
        ("Hide connection management window", "Verberg het venster voor verbindingsbeheer"),
        ("hide_cm_tip", "Dit kan alleen als de toegang via een permanent wachtwoord verloopt."),
        ("wayland_experiment_tip", "Wayland ondersteuning is slechts experimenteel. Gebruik alsjeblieft X11 als je onbeheerde toegang nodig hebt."),
        ("Right click to select tabs", "Rechts klikken om tabbladen te selecteren"),
        ("Skipped", "Overgeslagen"),
        ("Add to Address Book", "Toevoegen aan Adresboek"),
        ("Group", "Groep"),
        ("Search", "Zoek"),
        ("Closed manually by web console", "Handmatig gesloten door webconsole"),
        ("Local keyboard type", "Lokaal toetsenbord"),
        ("Select local keyboard type", "Selecteer lokaal toetsenbord"),
        ("software_render_tip", "Als u een NVIDIA grafische kaart hebt en het externe venster sluit onmiddellijk na verbinding, kan het helpen om het nieuwe stuurprogramma te installeren en te kiezen voor software rendering. Een software herstart is vereist."),
        ("Always use software rendering", "Gebruik altijd software rendering"),
        ("config_input", "Om het externe bureaublad vanaf het toetsenbord te kunnen bedienen, moet u RustDesk de rechten \"Invoerbewaking\" geven."),
        ("config_microphone", "Om op afstand te kunnen chatten, moet u RustDesk 'Audio opnemen' rechten geven."),
        ("request_elevation_tip", "U kunt ook meer rechten vragen als iemand aan de andere kant aanwezig is."),
        ("Wait", "Wacht"),
        ("Elevation Error", "Verhogingsfout"),
        ("Ask the remote user for authentication", "Vraag de gebruiker op afstand om bevestiging"),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", "De gebruiker op afstand moet altijd bevestigen via het UAC-venster van de werkende RustDesk."),
        ("Request Elevation", "Verzoek om meer rechten"),
        ("wait_accept_uac_tip", "Wacht tot de gebruiker op afstand het UAC-dialoogvenster accepteert."),
        ("Elevate successfully", "Succesvolle verhoging van privileges"),
        ("uppercase", "Hoofdletter"),
        ("lowercase", "kleine letter"),
        ("digit", "cijfer"),
        ("special character", "speciaal teken"),
        ("length>=8", "lengte>=8"),
        ("Weak", "Zwak"),
        ("Medium", "Midelmatig"),
        ("Strong", "Sterk"),
        ("Switch Sides", "Wissel van kant"),
        ("Please confirm if you want to share your desktop?", "bevestig als je je bureaublad wilt delen?"),
        ("Display", "Weergave"),
        ("Default View Style", "Standaard Weergave Stijl"),
        ("Default Scroll Style", "Standaard Scroll Stijl"),
        ("Default Image Quality", "Standaard Beeldkwaliteit"),
        ("Default Codec", "Standaard Codec"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Andere Standaardopties"),
        ("Voice call", "Spraakoproep"),
        ("Text chat", "Tekst chat"),
        ("Stop voice call", "Stop spraakoproep"),
        ("relay_hint_tip", "Indien een directe verbinding niet mogelijk is, kunt u proberen verbinding te maken via een Relay Server. \nAls u bij de eerste poging een relaisverbinding tot stand wilt brengen, kunt u het achtervoegsel \"/r\" toevoegen aan het ID of de optie \"Altijd verbinden via relaisserver\" selecteren op de externe terminal."),
        ("Reconnect", "Herverbinden"),
        ("Codec", "Codec"),
        ("Resolution", "Resolutie"),
        ("No transfers in progress", "Geen overdrachten in uitvoering"),
        ("Set one-time password length", "Stel de lengte van het eenmalige wachtwoord in"),
        ("install_cert_tip", ""),
        ("confirm_install_cert_tip", ""),
        ("RDP Settings", "RDP Instellingen"),
        ("Sort by", "Sorteren op"),
        ("New Connection", "Nieuwe Verbinding"),
        ("Restore", "Herstel"),
        ("Minimize", "Minimaliseren"),
        ("Maximize", "Maximaliseren"),
        ("Your Device", "Uw Apparaat"),
        ("empty_recent_tip", "Oeps, geen actuele situatie!\nTijd om een nieuwe te plannen."),
        ("empty_favorite_tip", "Nog geen favoriete Station op afstand? Laat ons iemand vinden om mee te verbinden en voeg hem toe aan je favorieten!"),
        ("empty_lan_tip", "Oh nee, het lijkt erop dat we nog geen extern station hebben ontdekt."),
        ("empty_address_book_tip", "Oh jee, het lijkt erop dat er momenteel geen externe stations in je adresboek staan."),
        ("eg: admin", "bijv: admin"),
        ("Empty Username", "Gebruikersnaam Leeg"),
        ("Empty Password", "Wachtwoord Leeg"),
        ("Me", "Ik"),
        ("identical_file_tip", "Dit bestand is identiek aan het bestand van het externe station."),
        ("show_monitors_tip", "Monitoren weergeven in de werkbalk"),
        ("View Mode", "Weergave Mode"),
        ("login_linux_tip", "Toegang tot het externe Linux-account"),
        ("verify_rustdesk_password_tip", "Bevestiging wachtwoord RustDesk"),
        ("remember_account_tip", "Herinner dit account"),
        ("os_account_desk_tip", "Dit account wordt gebruikt om toegang te krijgen tot het externe besturingssysteem en de bureaubladsessie in onbeheerde modus te activeren."),
        ("OS Account", "Besturingssysteem account"),
        ("another_user_login_title_tip", "Een andere gebruiker is al ingelogd."),
        ("another_user_login_text_tip", "Afzonderlijk"),
        ("xorg_not_found_title_tip", "Xorg niet gevonden."),
        ("xorg_not_found_text_tip", "Installeer Xorg."),
        ("no_desktop_title_tip", "Er is geen desktop beschikbaar."),
        ("no_desktop_text_tip", "Installeer de GNOME desktop."),
        ("No need to elevate", "Niet nodig om te verhogen"),
        ("System Sound", "Systeemgeluid"),
        ("Default", "Standaard"),
        ("New RDP", "Nieuwe RDP"),
        ("Fingerprint", "Vingerafdruk"),
        ("Copy Fingerprint", "Kopieer Vingerafdruk"),
        ("no fingerprints", "geen vingerafdrukken"),
        ("Select a peer", "Selecteer een peer"),
        ("Select peers", "Selecteer peers"),
        ("Plugins", "Plugins"),
        ("Uninstall", "Verwijder"),
        ("Update", "Bijwerken"),
        ("Enable", "Activeer"),
        ("Disable", "Deactiveer"),
        ("Options", "Opties"),
        ("resolution_original_tip", "Oorspronkelijke resolutie"),
        ("resolution_fit_local_tip", "Lokale resolutie aanpassen"),
        ("resolution_custom_tip", "Aangepaste resolutie"),
        ("Collapse toolbar", "Werkbalk samenvouwen"),
        ("Accept and Elevate", "Accepteren en Verheffen"),
        ("accept_and_elevate_btn_tooltip", "Accepteer de verbinding en verhoog de UAC-machtigingen."),
        ("clipboard_wait_response_timeout_tip", "Time-out in afwachting van kopieer-antwoord."),
        ("Incoming connection", "Inkomende verbinding"),
        ("Outgoing connection", "Uitgaande verbinding"),
        ("Exit", "Verlaten"),
        ("Open", "Open"),
        ("logout_tip", "Weet je zeker dat je je wilt afmelden?"),
        ("Service", "Service"),
        ("Start", "Start"),
        ("Stop", "Stop"),
        ("exceed_max_devices", "Het maximum aantal gecontroleerde apparaten is bereikt."),
        ("Sync with recent sessions", "Recente sessies synchroniseren"),
        ("Sort tags", "Labels sorteren"),
        ("Open new connections in tabs", ""),
        ("separate window", ""),
        ("Move tab to new window", ""),
    ].iter().cloned().collect();
}
