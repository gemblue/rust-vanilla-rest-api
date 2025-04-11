# 🦀 Rust Vanilla REST API

Latihan membangun REST API **tanpa framework** menggunakan Rust. Proyek ini bertujuan untuk memahami cara kerja HTTP dari nol, langsung dengan `TcpListener`, `serde`, dan `sqlite` untuk penyimpanan data.

> Belajar Rust sambil membongkar bagaimana request dan response bekerja tanpa bantuan framework apa pun. Cocok buat kamu yang mau paham "jeroan" backend secara low-level!

---

## 🚧 Fitur yang Sudah Dibuat

- [x] Menangani koneksi HTTP dasar (GET, POST, PUT, DELETE)
- [x] Parsing request HTTP manual
- [x] Response JSON dengan `serde_json`
- [x] Penyimpanan data dengan SQLite (opsional: bisa in-memory)
- [x] CRUD sederhana untuk data `Todo`

---

## 🧱 Teknologi

- `Rust` – bahasa utama
- `std::net::TcpListener` – buat dengerin koneksi
- `serde` & `serde_json` – untuk serialize / deserialize JSON
- `rusqlite` – SQLite driver untuk Rust

---

## 🏁 Cara Menjalankan

1. Clone repositori ini

   ```bash
   git clone https://github.com/username/rust-vanilla-rest-api.git
   cd rust-native-rest-api

2. Jalankan Server

```bash
cargo run

3. Uji endpoint menggunakan Postman / curl:

```bash
curl -X GET http://localhost:7878/todos

## 📬 Contoh Endpoint

| Method | Endpoint       | Deskripsi            |
|--------|----------------|----------------------|
| GET    | `/get`         | Get semua todo       |
| POST   | `/insert`      | Tambah todo baru     |
| PUT    | `/update`      | Update todo by id    |
| DELETE | `/todos/{id}`  | Hapus todo by id     |

Saya sudah buatkan Postman Collection, tinggal gunakan saja, unduh disini
https://drive.google.com/file/d/1qyv_kE1s67DNqK_Sz3rp_oJiqP31sK9_/view?usp=sharing
