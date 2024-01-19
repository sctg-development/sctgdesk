lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Desktop Anda"),
        ("desk_tip", "Desktop Anda dapat diakses dengan ID dan kata sandi ini."),
        ("Password", "Kata sandi"),
        ("Ready", "Sudah siap"),
        ("Established", "Didirikan"),
        ("connecting_status", "Menghubungkan ke jaringan RustDesk..."),
        ("Enable service", "Aktifkan Layanan"),
        ("Start service", "Mulai Layanan"),
        ("Service is running", "Layanan berjalan"),
        ("Service is not running", "Layanan tidak berjalan"),
        ("not_ready_status", "Belum siap. Silakan periksa koneksi Anda"),
        ("Control Remote Desktop", "Kontrol Remote Desktop"),
        ("Transfer file", "File Transfer"),
        ("Connect", "Hubungkan"),
        ("Recent sessions", "Sesi Terkini"),
        ("Address book", "Buku Alamat"),
        ("Confirmation", "Konfirmasi"),
        ("TCP tunneling", "Tunneling TCP"),
        ("Remove", "Hapus"),
        ("Refresh random password", "Perbarui kata sandi acak"),
        ("Set your own password", "Tetapkan kata sandi Anda"),
        ("Enable keyboard/mouse", "Aktifkan Keyboard/Mouse"),
        ("Enable clipboard", "Aktifkan Papan Klip"),
        ("Enable file transfer", "Aktifkan Transfer file"),
        ("Enable TCP tunneling", "Aktifkan TCP tunneling"),
        ("IP Whitelisting", "Daftar IP yang diizinkan"),
        ("ID/Relay Server", "ID/Relay Server"),
        ("Import server config", "Impor Konfigurasi Server"),
        ("Export Server Config", "Ekspor Konfigurasi Server"),
        ("Import server configuration successfully", "Impor konfigurasi server berhasil"),
        ("Export server configuration successfully", "Ekspor konfigurasi server berhasil"),
        ("Invalid server configuration", "Konfigurasi server tidak valid"),
        ("Clipboard is empty", "Papan klip kosong"),
        ("Stop service", "Hentikan Layanan"),
        ("Change ID", "Ubah ID"),
        ("Your new ID", "ID baru anda"),
        ("length %min% to %max%", "panjang %min% s/d %max%"),
        ("starts with a letter", "Dimulai dengan huruf"),
        ("allowed characters", "Karakter yang dapat digunakan"),
        ("id_change_tip", "Hanya karakter a-z, A-Z, 0-9 dan _ (underscore) yang diperbolehkan. Huruf pertama harus a-z, A-Z. Panjang antara 6 dan 16."),
        ("Website", "Situs Web"),
        ("About", "Tentang"),
        ("Slogan_tip", "Dibuat dengan penuh kasih sayang dalam dunia yang penuh kekacauan ini"),
        ("Privacy Statement", "Pernyataan Privasi"),
        ("Mute", "Bisukan"),
        ("Build Date", "Tanggal Build"),
        ("Version", "Versi"),
        ("Home", ""),
        ("Audio Input", "Input Audio"),
        ("Enhancements", "Peningkatan"),
        ("Hardware Codec", "Kodek Perangkat Keras"),
        ("Adaptive bitrate", "Kecepatan Bitrate Adaptif"),
        ("ID Server", "Server ID"),
        ("Relay Server", "Server Relay"),
        ("API Server", "Server API"),
        ("invalid_http", "harus dimulai dengan http:// atau https://"),
        ("Invalid IP", "IP tidak valid"),
        ("Invalid format", "Format tidak valid"),
        ("server_not_support", "Belum didukung oleh server"),
        ("Not available", "Tidak tersedia"),
        ("Too frequent", "Terlalu sering"),
        ("Cancel", "Batal"),
        ("Skip", "Lanjutkan"),
        ("Close", "Tutup"),
        ("Retry", "Coba lagi"),
        ("OK", "Oke"),
        ("Password Required", "Kata sandi tidak boleh kosong"),
        ("Please enter your password", "Silahkan masukkan kata sandi anda"),
        ("Remember password", "Ingat kata sandi"),
        ("Wrong Password", "Kata sandi Salah"),
        ("Do you want to enter again?", "Apakah anda ingin masuk lagi?"),
        ("Connection Error", "Kesalahan koneksi"),
        ("Error", "Kesalahan"),
        ("Reset by the peer", "Direset oleh rekan"),
        ("Connecting...", "Menghubungkan..."),
        ("Connection in progress. Please wait.", "Koneksi sedang berlangsung. Mohon tunggu."),
        ("Please try 1 minute later", "Silahkan coba 1 menit lagi"),
        ("Login Error", "Kesalahan Login"),
        ("Successful", "Berhasil"),
        ("Connected, waiting for image...", "Terhubung, menunggu gambar..."),
        ("Name", "Nama"),
        ("Type", "Tipe"),
        ("Modified", "Diperbarui"),
        ("Size", "Ukuran"),
        ("Show Hidden Files", "Tampilkan File Tersembunyi"),
        ("Receive", "Menerima"),
        ("Send", "Kirim"),
        ("Refresh File", "Segarkan File"),
        ("Local", "Lokal"),
        ("Remote", "Remote"),
        ("Remote Computer", "Remote Komputer"),
        ("Local Computer", "Lokal Komputer"),
        ("Confirm Delete", "Konfirmasi Hapus"),
        ("Delete", "Hapus"),
        ("Properties", "Properti"),
        ("Multi Select", "Pilih Beberapa"),
        ("Select All", "Pilih Semua"),
        ("Unselect All", "Batalkan Pilihan Semua"),
        ("Empty Directory", "Folder Kosong"),
        ("Not an empty directory", "Folder tidak kosong"),
        ("Are you sure you want to delete this file?", "Apakah anda yakin untuk menghapus file ini?"),
        ("Are you sure you want to delete this empty directory?", "Apakah anda yakin untuk menghapus folder ini?"),
        ("Are you sure you want to delete the file of this directory?", "Apakah anda yakin untuk menghapus file dan folder ini?"),
        ("Do this for all conflicts", "Lakukan untuk semua konflik"),
        ("This is irreversible!", "Ini tidak dapat diubah!"),
        ("Deleting", "Menghapus"),
        ("files", "file"),
        ("Waiting", "Menunggu"),
        ("Finished", "Selesai"),
        ("Speed", "Kecepatan"),
        ("Custom Image Quality", "Sesuaikan Kualitas Gambar"),
        ("Privacy mode", "Mode Privasi"),
        ("Block user input", "Blokir input pengguna"),
        ("Unblock user input", "Jangan blokir input pengguna"),
        ("Adjust Window", "Sesuaikan Jendela"),
        ("Original", "Asli"),
        ("Shrink", "Susutkan"),
        ("Stretch", "Regangkan"),
        ("Scrollbar", "Scrollbar"),
        ("ScrollAuto", "Scroll Otomatis"),
        ("Good image quality", "Kualitas Gambar Baik"),
        ("Balanced", "Seimbang"),
        ("Optimize reaction time", "Optimalkan waktu reaksi"),
        ("Custom", "Kustom"),
        ("Show remote cursor", "Tampilkan kursor remote"),
        ("Show quality monitor", "Tampilkan kualitas monitor"),
        ("Disable clipboard", "Matikan papan klip"),
        ("Lock after session end", "Kunci setelah sesi berakhir"),
        ("Insert", "Menyisipkan"),
        ("Insert Lock", "Masukkan Kunci"),
        ("Refresh", "Segarkan"),
        ("ID does not exist", "ID tidak ada"),
        ("Failed to connect to rendezvous server", "Gagal menghubungkan ke rendezvous server"),
        ("Please try later", "Silahkan coba lagi nanti"),
        ("Remote desktop is offline", "Remote desktop offline"),
        ("Key mismatch", "Ketidakcocokan kunci"),
        ("Timeout", "Waktu habis"),
        ("Failed to connect to relay server", "Gagal terkoneksi ke relay server"),
        ("Failed to connect via rendezvous server", "Gagal terkoneksi via rendezvous server"),
        ("Failed to connect via relay server", "Gagal terkoneksi via relay server"),
        ("Failed to make direct connection to remote desktop", "Gagal membuat koneksi langsung ke desktop jarak jauh"),
        ("Set Password", "Tetapkan kata sandi"),
        ("OS Password", "Kata Sandi OS"),
        ("install_tip", "Karena UAC, RustDesk tidak dapat bekerja dengan baik sebagai sisi remote dalam beberapa kasus. Untuk menghindari UAC, silakan klik tombol di bawah ini untuk menginstal RustDesk ke sistem."),
        ("Click to upgrade", "Klik untuk upgrade"),
        ("Click to download", "Klik untuk unduh"),
        ("Click to update", "Klik untuk memperbarui"),
        ("Configure", "Konfigurasi"),
        ("config_acc", "Untuk mengontrol Desktop Anda dari jarak jauh, Anda perlu memberikan izin \"Aksesibilitas\" RustDesk."),
        ("config_screen", "Untuk mengakses Desktop Anda dari jarak jauh, Anda perlu memberikan izin \"Perekaman Layar\" RustDesk."),
        ("Installing ...", "Menginstall"),
        ("Install", "Instal"),
        ("Installation", "Instalasi"),
        ("Installation Path", "Direktori Instalasi"),
        ("Create start menu shortcuts", "Buat pintasan start menu"),
        ("Create desktop icon", "Buat icon desktop"),
        ("agreement_tip", "Dengan memulai instalasi, Anda menerima perjanjian lisensi."),
        ("Accept and Install", "Terima dan Install"),
        ("End-user license agreement", "Perjanjian lisensi pengguna"),
        ("Generating ...", "Memproses..."),
        ("Your installation is lower version.", "Instalasi Anda adalah versi yang lebih rendah."),
        ("not_close_tcp_tip", "Jangan tutup jendela ini saat menggunakan tunnel"),
        ("Listening ...", "Menghubungkan..."),
        ("Remote Host", "Host Remote"),
        ("Remote Port", "Port Remote"),
        ("Action", "Aksi"),
        ("Add", "Tambah"),
        ("Local Port", "Port Lokal"),
        ("Local Address", "Alamat lokal"),
        ("Change Local Port", "Ubah Port Lokal"),
        ("setup_server_tip", "Untuk mendapatkan koneksi yang lebih baik, disarankan untuk menginstal di server anda sendiri"),
        ("Too short, at least 6 characters.", "Terlalu pendek, setidaknya 6 karekter."),
        ("The confirmation is not identical.", "Konfirmasi tidak identik."),
        ("Permissions", "Perizinan"),
        ("Accept", "Terima"),
        ("Dismiss", "Hentikan"),
        ("Disconnect", "Terputus"),
        ("Enable file copy and paste", "Izinkan salin dan tempel file"),
        ("Connected", "Terhubung"),
        ("Direct and encrypted connection", "Koneksi langsung dan terenkripsi"),
        ("Relayed and encrypted connection", "Koneksi relay dan terenkripsi"),
        ("Direct and unencrypted connection", "Koneksi langsung dan tanpa enkripsi"),
        ("Relayed and unencrypted connection", "Koneksi relay dan tanpa enkripsi"),
        ("Enter Remote ID", "Masukkan ID Remote"),
        ("Enter your password", "Masukkan kata sandi anda"),
        ("Logging in...", "Masuk..."),
        ("Enable RDP session sharing", "Aktifkan berbagi sesi RDP"),
        ("Auto Login", "Login Otomatis (Hanya berlaku jika Anda mengatur \"Kunci setelah sesi berakhir\")"),
        ("Enable direct IP access", "Aktifkan Akses IP Langsung"),
        ("Rename", "Ubah nama"),
        ("Space", "Spasi"),
        ("Create desktop shortcut", "Buat Pintasan Desktop"),
        ("Change Path", "Ubah Direktori"),
        ("Create Folder", "Buat Folder"),
        ("Please enter the folder name", "Silahkan masukkan nama folder"),
        ("Fix it", "Perbaiki"),
        ("Warning", "Peringatan"),
        ("Login screen using Wayland is not supported", "Layar masuk menggunakan Wayland tidak didukung"),
        ("Reboot required", "Diperlukan boot ulang"),
        ("Unsupported display server", "Server tampilan tidak didukung "),
        ("x11 expected", "Diperlukan x11"),
        ("Port", "Port"),
        ("Settings", "Pengaturan"),
        ("Username", "Nama pengguna"),
        ("Invalid port", "Kesalahan port"),
        ("Closed manually by the peer", "Ditutup secara manual oleh rekan"),
        ("Enable remote configuration modification", "Aktifkan modifikasi konfigurasi remotE"),
        ("Run without install", "Jalankan tanpa menginstal"),
        ("Connect via relay", "Sambungkan via relay"),
        ("Always connect via relay", "Selalu terhubung melalui relay"),
        ("whitelist_tip", "Hanya IP yang diizikan dapat mengakses"),
        ("Login", "Masuk"),
        ("Verify", "Verifikasi"),
        ("Remember me", "Ingatkan saya"),
        ("Trust this device", "Izinkan perangkat ini"),
        ("Verification code", "Kode verifikasi"),
        ("verification_tip", "Kode verifikasi sudah dikirim ke email yang terdaftar, masukkan kode verifikasi untuk melanjutkan."),
        ("Logout", "Keluar"),
        ("Tags", "Tag"),
        ("Search ID", "Cari ID"),
        ("whitelist_sep", "Dipisahkan dengan koma, titik koma, spasi, atau baris baru"),
        ("Add ID", "Tambah ID"),
        ("Add Tag", "Tambah Tag"),
        ("Unselect all tags", "Batalkan pilihan semua tag"),
        ("Network error", "Kesalahan Jaringan"),
        ("Username missed", "Nama pengguna tidak sesuai"),
        ("Password missed", "Kata sandi tidak sesuai"),
        ("Wrong credentials", "Nama pengguna atau kata sandi salah"),
        ("The verification code is incorrect or has expired", "Kode verifikasi salah atau sudah kadaluarsa"),
        ("Edit Tag", "Ubah Tag"),
        ("Forget Password", "Lupakan Kata Sandi"),
        ("Favorites", "Favorit"),
        ("Add to Favorites", "Tambah ke Favorit"),
        ("Remove from Favorites", "Hapus dari favorit"),
        ("Empty", "Kosong"),
        ("Invalid folder name", "Nama folder tidak valid"),
        ("Socks5 Proxy", "Proksi Socks5"),
        ("Hostname", "Hostname"),
        ("Discovered", "Telah ditemukan"),
        ("install_daemon_tip", "Untuk memulai saat boot, Anda perlu menginstal system service."),
        ("Remote ID", "ID Remote"),
        ("Paste", "Tempel"),
        ("Paste here?", "Tempel disini?"),
        ("Are you sure to close the connection?", "Apakah anda yakin akan menutup koneksi?"),
        ("Download new version", "Unduh versi baru"),
        ("Touch mode", "Mode Layar Sentuh"),
        ("Mouse mode", "Mode Mouse"),
        ("One-Finger Tap", "Ketuk Satu Jari"),
        ("Left Mouse", "Mouse Kiri"),
        ("One-Long Tap", "Ketuk Tahan"),
        ("Two-Finger Tap", "Ketuk Dua Jari"),
        ("Right Mouse", "Mouse Kanan"),
        ("One-Finger Move", "Gerakan Satu Jari"),
        ("Double Tap & Move", "Ketuk Dua Kali & Pindah"),
        ("Mouse Drag", "Geser Mouse"),
        ("Three-Finger vertically", "Tiga Jari secara vertikal"),
        ("Mouse Wheel", "Roda mouse"),
        ("Two-Finger Move", "Gerakan Dua Jari"),
        ("Canvas Move", "Gerakan Kanvas"),
        ("Pinch to Zoom", "Cubit untuk Memperbesar"),
        ("Canvas Zoom", "Perbesar Canvas"),
        ("Reset canvas", "Setel Ulang Canvas"),
        ("No permission of file transfer", "Tidak ada izin untuk mengirim file"),
        ("Note", "Catatan"),
        ("Connection", "Koneksi"),
        ("Share Screen", "Bagikan Layar"),
        ("Chat", "Obrolan"),
        ("Total", "Total"),
        ("items", "item"),
        ("Selected", "Dipilih"),
        ("Screen Capture", "Tangkapan Layar"),
        ("Input Control", "Kontrol input"),
        ("Audio Capture", "Rekam Suara"),
        ("File Connection", "Koneksi File"),
        ("Screen Connection", "Koneksi layar"),
        ("Do you accept?", "Apakah anda setuju?"),
        ("Open System Setting", "Buka Pengaturan Sistem"),
        ("How to get Android input permission?", "Bagaimana cara mendapatkan izin input dari Android?"),
        ("android_input_permission_tip1", "Agar perangkat jarak jauh dapat mengontrol perangkat Android Anda melalui mouse atau sentuhan, Anda harus mengizinkan RustDesk untuk menggunakan layanan \"Aksesibilitas\"."),
        ("android_input_permission_tip2", "Silakan buka halaman pengaturan sistem berikutnya, temukan dan masuk ke [Layanan Terinstal], aktifkan layanan [Input RustDesk]."),
        ("android_new_connection_tip", "Permintaan akses remote telah diterima"),
        ("android_service_will_start_tip", "Mengaktifkan \"Tangkapan Layar\" akan memulai secara otomatis, memungkinkan perangkat lain untuk meminta koneksi ke perangkat Anda."),
        ("android_stop_service_tip", "Menutup layanan secara otomatis akan menutup semua koneksi yang dibuat."),
        ("android_version_audio_tip", "Versi Android saat ini tidak mendukung pengambilan audio, harap tingkatkan ke Android 10 atau lebih tinggi."),
        ("android_start_service_tip", "Tap [Mulai Layanan] atau aktifkan izin [Tangkapan Layar] untuk memulai berbagi layar."),
        ("android_permission_may_not_change_tip", "Izin untuk koneksi yang sudah terhubung mungkin tidak dapat diubah secara instan hingga terhubung kembali"),
        ("Account", "Akun"),
        ("Overwrite", "Ganti"),
        ("This file exists, skip or overwrite this file?", "File ini sudah ada, lewati atau ganti file ini?"),
        ("Quit", "Keluar"),
        ("Help", "Bantuan"),
        ("Failed", "Gagal"),
        ("Succeeded", "Berhasil"),
        ("Someone turns on privacy mode, exit", "Seseorang mengaktifkan mode privasi, keluar"),
        ("Unsupported", "Tidak didukung"),
        ("Peer denied", "Rekan menolak"),
        ("Please install plugins", "Silakan instal plugin"),
        ("Peer exit", "Rekan keluar"),
        ("Failed to turn off", "Gagal mematikan"),
        ("Turned off", "Dimatikan"),
        ("Language", "Bahasa"),
        ("Keep RustDesk background service", "Pertahankan RustDesk berjalan pada service background"),
        ("Ignore Battery Optimizations", "Abaikan Pengoptimalan Baterai"),
        ("android_open_battery_optimizations_tip", "Jika anda ingin menonaktifkan fitur ini, buka halam pengaturan, cari dan pilih [Baterai], Uncheck [Tidak dibatasi]"),
        ("Start on boot", "Mulai saat dihidupkan"),
        ("Start the screen sharing service on boot, requires special permissions", "Mulai layanan berbagi layar saat sistem dinyalakan, memerlukan izin khusus."),
        ("Connection not allowed", "Koneksi tidak dizinkan"),
        ("Legacy mode", "Mode lawas"),
        ("Map mode", "Mode peta"),
        ("Translate mode", "Mode terjemahan"),
        ("Use permanent password", "Gunakan kata sandi permanaen"),
        ("Use both passwords", "Gunakan kedua kata sandi"),
        ("Set permanent password", "Setel kata sandi permanen"),
        ("Enable remote restart", "Aktifkan Restart Secara Remote"),
        ("Restart remote device", "Restart Perangkat Secara Remote"),
        ("Are you sure you want to restart", "Apakah Anda yakin ingin merestart"),
        ("Restarting remote device", "Merestart Perangkat Remote"),
        ("remote_restarting_tip", "Perangkat remote sedang merestart, harap tutup pesan ini dan sambungkan kembali dengan kata sandi permanen setelah beberapa saat."),
        ("Copied", "Disalin"),
        ("Exit Fullscreen", "Keluar dari Layar Penuh"),
        ("Fullscreen", "Layar penuh"),
        ("Mobile Actions", "Tindakan Seluler"),
        ("Select Monitor", "Pilih Monitor"),
        ("Control Actions", "Tindakan Kontrol"),
        ("Display Settings", "Pengaturan tampilan"),
        ("Ratio", "Rasio"),
        ("Image Quality", "Kualitas gambar"),
        ("Scroll Style", "Gaya Scroll"),
        ("Show Toolbar", "Tampilkan Toolbar"),
        ("Hide Toolbar", "Sembunyikan Toolbar"),
        ("Direct Connection", "Koneksi langsung"),
        ("Relay Connection", "Koneksi Relay"),
        ("Secure Connection", "Koneksi aman"),
        ("Insecure Connection", "Koneksi Tidak Aman"),
        ("Scale original", "Skala asli"),
        ("Scale adaptive", "Skala adaptif"),
        ("General", "Umum"),
        ("Security", "Keamanan"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Gelap"),
        ("Light Theme", "Tema Terang"),
        ("Dark", "Gelap"),
        ("Light", "Terang"),
        ("Follow System", "Ikuti Sistem"),
        ("Enable hardware codec", "Aktifkan kodek perangkat keras"),
        ("Unlock Security Settings", "Buka Keamanan Pengaturan"),
        ("Enable audio", "Aktifkan Audio"),
        ("Unlock Network Settings", "Buka Keamanan Pengaturan Jaringan"),
        ("Server", "Server"),
        ("Direct IP Access", "Akses IP Langsung"),
        ("Proxy", "Proksi"),
        ("Apply", "Terapkan"),
        ("Disconnect all devices?", "Putuskan sambungan semua perangkat?"),
        ("Clear", "Bersihkan"),
        ("Audio Input Device", "Input Perangkat Audio"),
        ("Use IP Whitelisting", "Gunakan daftar IP yang diizinkan"),
        ("Network", "Jaringan"),
        ("Pin Toolbar", "Sematkan Toolbar"),
        ("Unpin Toolbar", "Batal sematkan Toolbar"),
        ("Recording", "Perekaman"),
        ("Directory", "Direktori"),
        ("Automatically record incoming sessions", "Otomatis merekam sesi masuk"),
        ("Change", "Ubah"),
        ("Start session recording", "Mulai sesi perekaman"),
        ("Stop session recording", "Hentikan sesi perekaman"),
        ("Enable recording session", "Aktifkan Sesi Perekaman"),
        ("Enable LAN discovery", "Aktifkan Pencarian Jaringan Lokal (LAN)"),
        ("Deny LAN discovery", "Tolak Pencarian Jaringan Lokal (LAN)"),
        ("Write a message", "Tulis pesan"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", "Harap tunggu konfirmasi UAC"),
        ("elevated_foreground_window_tip", "Jendela remote desktop ini memerlukan hak akses khusus, jadi anda tidak bisa menggunakan mouse dan keyboard untuk sementara. Anda bisa meminta pihak pengguna yang diremote untuk menyembunyikan jendela ini atau klik tombol elevasi di jendela pengaturan koneksi. Untuk menghindari masalah ini, direkomendasikan untuk menginstall aplikasi secara permanen"),
        ("Disconnected", "Terputus"),
        ("Other", "Lainnya"),
        ("Confirm before closing multiple tabs", "Konfirmasi sebelum menutup banyak tab"),
        ("Keyboard Settings", "Pengaturan Papan Ketik"),
        ("Full Access", "Akses penuh"),
        ("Screen Share", "Berbagi Layar"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland membutuhkan Ubuntu 21.04 atau versi yang lebih tinggi."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland membutuhkan versi distro linux yang lebih tinggi. Silakan coba desktop X11 atau ubah OS Anda."),
        ("JumpLink", "Tautan Cepat"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Silakan Pilih layar yang akan dibagikan kepada rekan anda."),
        ("Show RustDesk", "Tampilkan RustDesk"),
        ("This PC", "PC ini"),
        ("or", "atau"),
        ("Continue with", "Lanjutkan dengan"),
        ("Elevate", "Elevasi"),
        ("Zoom cursor", "Perbersar Kursor"),
        ("Accept sessions via password", "Izinkan sesi dengan kata sandi"),
        ("Accept sessions via click", "Izinkan sesi dengan klik"),
        ("Accept sessions via both", "Izinkan sesi dengan keduanya"),
        ("Please wait for the remote side to accept your session request...", "Harap tunggu pihak pengguna remote untuk menerima permintaan sesi..."),
        ("One-time Password", "Kata sandi sekali pakai"),
        ("Use one-time password", "Gunakan kata sandi sekali pakai"),
        ("One-time password length", "Panjang kata sandi sekali pakai"),
        ("Request access to your device", "Permintaan akses ke perangkat ini"),
        ("Hide connection management window", "Sembunyikan jendela pengaturan koneksi"),
        ("hide_cm_tip", "Izinkan untuk menyembunyikan hanya jika menerima sesi melalui kata sandi dan menggunakan kata sandi permanen"),
        ("wayland_experiment_tip", "Dukungan Wayland masih dalam tahap percobaan, harap gunakan X11 jika Anda memerlukan akses tanpa pengawasan"),
        ("Right click to select tabs", "Klik kanan untuk memilih tab"),
        ("Skipped", "Dilewati"),
        ("Add to address book", "Tambahkan ke Buku Alamat"),
        ("Group", "Grup"),
        ("Search", "Pencarian"),
        ("Closed manually by web console", "Ditutup secara manual dari konsol web."),
        ("Local keyboard type", "Tipe papan ketik"),
        ("Select local keyboard type", "Pilih tipe papan ketik"),
        ("software_render_tip", "Jika anda menggunakan kartu grafis Nvidia pada sistem linux dan jendela windows ditutup secara instan setelah terhung, silahkan ubah ke driver open-source Nouveau, dibutukan untuk merestart aplikasi"),
        ("Always use software rendering", "Selalu gunakan software rendering"),
        ("config_input", "Untuk menggunakan input keyboard remote, anda perlu memberikan izin \"Pemantauan Input\" pada RustDesk"),
        ("config_microphone", "Untuk berbicara secara remote, anda perlu memberikan izin \"Rekam Audio\" pada RustDesk"),
        ("request_elevation_tip", "Anda juga bisa meminta izin elevasi jika ada pihak pengguna remote"),
        ("Wait", "Tunggu"),
        ("Elevation Error", "Kesalahan Elevasi"),
        ("Ask the remote user for authentication", "Minta pihak pengguna remote untuk otentikasi"),
        ("Choose this if the remote account is administrator", "Pilih ini jika akun adalah \"administrator\""),
        ("Transmit the username and password of administrator", "Transmisikan nama pengguna dan kata sandi administrator"),
        ("still_click_uac_tip", "Masih memerlukan persetujuan pihak pengguna remote untuk mengklik OK pada jendela UAC RustDesk yang sedang berjalan"),
        ("Request Elevation", "Permintaan Elevasi"),
        ("wait_accept_uac_tip", "Harap tunggu pihak pengguna remote menerima jendela UAC."),
        ("Elevate successfully", "Elevasi berhasil"),
        ("uppercase", "Huruf besar"),
        ("lowercase", "Huruf kecil"),
        ("digit", "angka"),
        ("special character", "Karakter spesial"),
        ("length>=8", "panjang>=8"),
        ("Weak", "Lemah"),
        ("Medium", "Sedang"),
        ("Strong", "Kuat"),
        ("Switch Sides", "Ganti Posisi"),
        ("Please confirm if you want to share your desktop?", "Harap konfirmasi apakah Anda ingin berbagi layar?"),
        ("Display", "Tampilan"),
        ("Default View Style", "Gaya Tampilan Default"),
        ("Default Scroll Style", "Gaya Scroll Default"),
        ("Default Image Quality", "Kualitas Gambar Default"),
        ("Default Codec", "Kodek default"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Otomatis"),
        ("Other Default Options", "Opsi Default Lainnya"),
        ("Voice call", "Panggilan suara"),
        ("Text chat", "Obrolan Teks"),
        ("Stop voice call", "Hentikan panggilan suara"),
        ("relay_hint_tip", "Tidak memungkinkan untuk terhubung secara langsung; anda bisa mencoba terhubung via relay. Selain itu, jika ingin menggunakan relay pada percobaan pertama, silahkan tambah akhiran \"/r\" pada ID atau pilih \"Selalu terhubung via relay\" di pilihan sesi terbaru."),
        ("Reconnect", "Sambungkan ulang"),
        ("Codec", "Kodek"),
        ("Resolution", "Resolusi"),
        ("No transfers in progress", "Tidak ada proses transfer data"),
        ("Set one-time password length", "Atur panjang kata sandi sekali pakai"),
        ("install_cert_tip", "Install sertifikat RustDesk"),
        ("confirm_install_cert_tip", "Ini adalah sertifikat pengujian RustDesk, yang dapat dipercaya. Sertifikat ini akan digunakan untuk menginstal driver RustDesk saat diperlukan"),
        ("RDP Settings", "Pengaturan RDP"),
        ("Sort by", "Urutkan berdasarkan"),
        ("New Connection", "Koneksi baru"),
        ("Restore", "Mengembalikan"),
        ("Minimize", "Meminimalkan"),
        ("Maximize", "Memaksimalkan"),
        ("Your Device", "Perangkat anda"),
        ("empty_recent_tip", "Tidak ada sesi terbaru!"),
        ("empty_favorite_tip", "Belum ada rekan favorit?\nTemukan seseorang untuk terhubung dan tambahkan ke favorit!"),
        ("empty_lan_tip", "Sepertinya kami belum memiliki rekan"),
        ("empty_address_book_tip", "Tampaknya saat ini tidak ada rekan yang terdaftar dalam buku alamat Anda"),
        ("eg: admin", "contoh: admin"),
        ("Empty Username", "Nama pengguna kosong"),
        ("Empty Password", "Kata sandi kosong"),
        ("Me", "Saya"),
        ("identical_file_tip", "Data ini identik dengan milik rekan"),
        ("show_monitors_tip", "Tampilkan monitor di toolbar"),
        ("View Mode", "Mode Tampilan"),
        ("login_linux_tip", "Anda harus masuk ke akun remote linux untuk mengaktifkan sesi X desktop"),
        ("verify_rustdesk_password_tip", "Verifikasi Kata Sandi RustDesk"),
        ("remember_account_tip", "Ingat akun ini"),
        ("os_account_desk_tip", "Akun ini digunakan untuk masuk ke sistem operasi remote dan mengaktifkan sesi desktop dalam mode tanpa tampilan (headless)"),
        ("OS Account", "Akun OS"),
        ("another_user_login_title_tip", "Akun ini sedang digunakan"),
        ("another_user_login_text_tip", "Putuskan koneksi diperangkat lain"),
        ("xorg_not_found_title_tip", "Xorg tidak ditemukan"),
        ("xorg_not_found_text_tip", "Silahkan install Xorg"),
        ("no_desktop_title_tip", "Desktop tidak tersedia"),
        ("no_desktop_text_tip", "Silahkan install GNOME Desktop"),
        ("No need to elevate", "Tidak perlu elevasi"),
        ("System Sound", "Suara Sistem"),
        ("Default", "Default"),
        ("New RDP", "RDP Baru"),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", "Pilih rekan"),
        ("Select peers", "Pilih rekan-rekan"),
        ("Plugins", "Plugin"),
        ("Uninstall", "Hapus instalasi"),
        ("Update", "Perbarui"),
        ("Enable", "Aktifkan"),
        ("Disable", "Nonaktifkan"),
        ("Options", "Opsi"),
        ("resolution_original_tip", "Resolusi original"),
        ("resolution_fit_local_tip", "Sesuaikan resolusi lokal"),
        ("resolution_custom_tip", "Resolusi kustom"),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", "Terima dan Elevasi"),
        ("accept_and_elevate_btn_tooltip", "Terima koneksi dan elevasi izin UAC"),
        ("clipboard_wait_response_timeout_tip", "Batas waktu habis saat menunggu respons salinan"),
        ("Incoming connection", "Koneksi akan masuk"),
        ("Outgoing connection", "Koneksi akan keluar"),
        ("Exit", "Keluar"),
        ("Open", "Buka"),
        ("logout_tip", "Apakah Anda yakin ingin keluar?"),
        ("Service", "Service"),
        ("Start", "Jalankan"),
        ("Stop", "Hentikan"),
        ("exceed_max_devices", "Anda telah mencapai jumlah maksimal perangkat yang dikelola"),
        ("Sync with recent sessions", "Sinkronkan dengan sesi terbaru"),
        ("Sort tags", "Urutkan tag"),
        ("Open connection in new tab", "Buka koneksi di tab baru"),
        ("Move tab to new window", "Pindahkan tab ke jendela baru"),
        ("Can not be empty", "Tidak boleh kosong"),
        ("Already exists", "Sudah ada"),
        ("Change Password", "Ganti kata sandi"),
        ("Refresh Password", "Perbarui Kata Sandi"),
        ("ID", "ID"),
        ("Grid View", "Tampilan Kotak"),
        ("List View", "Tampilan Daftar"),
        ("Select", "Pilih"),
        ("Toggle Tags", "Toggle Tag"),
        ("pull_ab_failed_tip", "Gagal memuat ulang buku alamat"),
        ("push_ab_failed_tip", "Gagal menyinkronkan buku alamat ke server"),
        ("synced_peer_readded_tip", "Perangkat yang terdaftar dalam sesi terbaru akan di-sinkronkan kembali ke buku alamat."),
        ("Change Color", "Ganti warna"),
        ("Primary Color", "Warna utama"),
        ("HSV Color", "Warna HSV"),
        ("Installation Successful!", "Instalasi berhasil!"),
        ("Installation failed!", "Instalasi gagal!"),
        ("Reverse mouse wheel", "Balikkan arah scroll mouse"),
        ("{} sessions", "sesi {}"),
        ("scam_title", "Kemungkinan anda Sedang DITIPU!"),
        ("scam_text1", "Jika anda sedang berbicara di telepon dengan seseorang yang TIDAK dikenal dan mereka meminta anda untuk menggunakan RustDesk, jangan lanjutkan dan segera tutup panggilan."),
        ("scam_text2", "Kemungkinan besar mereka adalah komplotan penipu yang berusaha mencuri uang atau informasi pribadi anda."),
        ("Don't show again", "Jangan tampilkan lagi"),
        ("I Agree", "Saya setuju"),
        ("Decline", "Tolak"),
        ("Timeout in minutes", "Batasan Waktu dalam Menit"),
        ("auto_disconnect_option_tip", "Secara otomatis akan menutup sesi ketika tidak ada aktivitas"),
        ("Connection failed due to inactivity", "Secara otomatis akan terputus ketik tidak ada aktivitas."),
        ("Check for software update on startup", "Periksa pembaruan aplikasi saat sistem dinyalakan."),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Silahkan perbarui RustDesk Server Pro ke versi {} atau yang lebih baru!"),
        ("pull_group_failed_tip", "Gagal memperbarui grup"),
        ("Filter by intersection", "Filter berdasarkan persilangan"),
        ("Remove wallpaper during incoming sessions", "Hilangkan latar dinding ketika ada sesi yang masuk"),
        ("Test", "Tes"),
        ("display_is_plugged_out_msg", "Layar terputus, pindah ke layar pertama"),
        ("No displays", "Tidak ada tampilan"),
        ("elevated_switch_display_msg", "Pindah ke tampilan utama, pada mode elevasi, pengggunaan lebih dari satu layar tidak diizinkan"),
        ("Open in new window", "Buka di jendela baru"),
        ("Show displays as individual windows", "Tampilkan dengan jendela terpisah"),
        ("Use all my displays for the remote session", "Gunakan semua layar untuk sesi remote"),
        ("selinux_tip", "pada perangkat anda, SELinux sedang aktif, yang mana itu dapat mencegah RustDesk berfungsi dengan baik sebagai sisi yang dikontrol."),
        ("Change view", "Sesuaikan tampilan"),
        ("Big tiles", "Kotak besar"),
        ("Small tiles", "Kotak kecil"),
        ("List", "Daftar"),
        ("Virtual display", "Tampilan virtual"),
        ("Plug out all", "Lepaskan semua"),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", "Aktifkan pemblokiran input pengguna"),
        ("id_input_tip", "Anda bisa memasukkan ID, IP langsung, atau domain dengan port kostum yang sudah ditentukan (<domain>:<port>).\nJika anda ingin mengakses perangkat lain yang berbeda server, tambahkan alamat server setelah penulisan ID(<id>@<server_address>?key=<key_value>), sebagai contoh,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nJika anda ingin mengakses perangkat yang menggunakan server publik, masukkan \"<id>@public\", server public tidak memerlukan key khusus"),
        ("privacy_mode_impl_mag_tip", "Mode 1"),
        ("privacy_mode_impl_virtual_display_tip", "Mode 2"),
        ("Enter privacy mode", "Masuk mode privasi"),
        ("Exit privacy mode", "Keluar mode privasi"),
        ("idd_not_support_under_win10_2004_tip", "Driver grafis yang Anda gunakan tidak kompatibel dengan versi Windows Anda dan memerlukan Windows 10 versi 2004 atau yang lebih baru"),
        ("switch_display_elevated_connections_tip", "Pada mode elevasi, jika terdapat beberapa tampilan yang aktif, maka tidak diizinkan berpindah ke yang bukan tampilan utama, silahkan coba lagi setelah proses instalasi jika kamu ingin melakukan kontrol ke tampilan layar lainnya"),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("2fa_tip", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
    ].iter().cloned().collect();
}
