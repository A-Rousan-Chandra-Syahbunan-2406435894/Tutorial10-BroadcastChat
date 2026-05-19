# Tutorial 2: Broadcast Chat

## Experiment 2.1: Original code, and how it run
![alt text](image.png)
![alt text](image-1.png)
![alt text](image-2.png)
![alt text](image-3.png)

**Cara menjalankan program:**

1. Buka satu terminal, masuk ke direktori project, lalu jalankan server:
   ```bash
   cargo run --bin server

2. Buka tiga terminal baru secara terpisah, lalu jalankan client di masing-masing terminal tersebut:
   ```bash
   cargo run --bin client

3. Ketik pesan di salah satu terminal client dan tekan Enter. Pesan tersebut akan muncul di semua terminal client lainnya yang sedang terhubung ke server.

## Penjelasan

Pada tutorial ini, saya menjalankan satu buah server dan tiga buah client secara paralel di terminal yang terpisah. Saat saya menginput dan mengirimkan teks dari salah satu terminal client, pesan tersebut langsung diteruskan ke server. Setelah menerimanya, server seketika melakukan proses broadcast (siaran) dengan mendistribusikan pesan tersebut ke semua terminal client lain yang saat itu berstatus connected. Hasilnya, obrolan dari satu pengguna dapat dibaca oleh seluruh pengguna lainnya secara real-time.

Beberapa poin penting yang terjadi di belakang layar:

1. Konkurensi Tinggi: Server tidak bekerja secara sekuensial (menunggu satu client selesai baru melayani client lain), melainkan mampu memproses lalu lintas data dari banyak client secara bersamaan (concurrently) tanpa memblokir proses lainnya.

2. Protokol WebSocket: Penggunaan WebSockets (berbeda dengan HTTP biasa) menjaga koneksi ("pipa komunikasi") antara server dan tiap client tetap terbuka terus-menerus (persistent). Ini memungkinkan aliran data dua arah (mengirim dan menerima) yang sangat cepat kapan pun dibutuhkan.

3. Peran Async Runtime: Kehadiran runtime asinkron (seperti Tokio di Rust) adalah kunci utama yang membuat server tetap berjalan ringan dan sangat responsif, meskipun harus me-l-routing pesan ke berbagai soket koneksi di waktu yang hampir bersamaan.
