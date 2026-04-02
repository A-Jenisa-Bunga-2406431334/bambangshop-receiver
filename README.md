### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

1. RwLock<> diperlukan untuk mensinkronisasi akses ke Vec of Notifications 
karena Vec tidak thread-safe secara default di Rust. Kita menggunakan RwLock 
bukan Mutex karena RwLock memungkinkan multiple reader mengakses data secara 
bersamaan, sementara hanya satu writer yang bisa mengakses data pada satu 
waktu. Ini sangat tepat untuk kasus kita karena operasi list_all_as_string() 
(read) jauh lebih sering dipanggil dibandingkan operasi add() (write). Jika 
menggunakan Mutex, hanya satu thread yang bisa mengakses data pada satu waktu 
meskipun operasinya hanya membaca, yang akan mengurangi performa aplikasi 
secara signifikan karena setiap request GET harus menunggu giliran meskipun 
tidak ada yang sedang menulis data.

2. Rust tidak mengizinkan mutasi variabel static secara langsung karena 
alasan keamanan memori dan thread safety yang ketat. Di Java, variabel static 
bisa dimutasi melalui static function karena Java menggunakan garbage collector 
dan runtime checks untuk mengelola memori. Namun Rust menggunakan sistem 
ownership dan borrowing yang diverifikasi saat compile time. Jika Rust 
mengizinkan mutasi static variable secara langsung, tidak ada jaminan thread 
safety karena multiple threads bisa mengakses dan memodifikasi variabel yang 
sama secara bersamaan, menyebabkan data race. Oleh karena itu, kita 
menggunakan lazy_static untuk inisialisasi lazy dan RwLock atau DashMap untuk 
memastikan akses yang thread-safe ke variabel static tersebut.

#### Reflection Subscriber-2

1. Ya, saya mengeksplorasi src/lib.rs dan menemukan banyak hal menarik. 
Di sana terdapat konfigurasi APP_CONFIG yang menyimpan instance_root_url, 
publisher_root_url, dan instance_name yang dibaca dari environment variables 
melalui file .env. Selain itu terdapat REQWEST_CLIENT sebagai singleton HTTP 
client yang di-share ke seluruh aplikasi, sehingga tidak perlu membuat HTTP 
client baru setiap kali melakukan request. Pola ini sangat efisien karena 
menghindari overhead pembuatan koneksi berulang kali. Saya juga belajar 
bagaimana Rust menggunakan trait Getters dari crate getset untuk 
auto-generate getter methods pada struct.

2. Observer pattern sangat memudahkan penambahan subscriber baru. Untuk 
menambah instance Receiver baru, kita hanya perlu menjalankan instance baru 
dengan port berbeda (mengubah ROCKET_PORT di .env) dan melakukan subscribe 
ke product type yang diinginkan. Publisher tidak perlu dimodifikasi sama 
sekali karena ia hanya menyimpan list URL subscriber dan mengirim notifikasi 
ke semua URL tersebut. Untuk menambah lebih dari satu instance Main app juga 
relatif mudah karena setiap instance Main app memiliki list subscriber-nya 
sendiri. Namun tantangannya adalah sinkronisasi data antar instance Main app 
jika kita ingin semua instance memiliki data subscriber yang sama, yang 
memerlukan shared database atau message broker seperti Redis.

3. Saya mencoba membuat automated tests di Postman menggunakan fitur Tests 
tab yang memungkinkan kita menulis JavaScript untuk memverifikasi response. 
Misalnya, memverifikasi bahwa status code adalah 200, response body berisi 
field tertentu, atau nilai field sesuai ekspektasi. Fitur ini sangat berguna 
untuk Group Project karena kita bisa membuat test suite yang otomatis 
memverifikasi semua endpoint API tanpa perlu melakukan pengecekan manual 
setiap kali ada perubahan kode. Saya juga menggunakan fitur Collection Runner 
untuk menjalankan semua test sekaligus dan mendapatkan laporan hasilnya.