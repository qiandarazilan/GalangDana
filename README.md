# Galang Dana

Aplikasi penggalangan dana berbasis Rust yang berjalan di jaringan ICP.

## Fitur

- **Pengguna:**
  - Membuat kampanye penggalangan dana.
  - Menyumbang ke kampanye tertentu.
  - Melihat status pendanaan kampanye.
- **Admin:**
  - Verifikasi kampanye.
  - Mengelola daftar kampanye.

## Cara Menjalankan

### Backend

1. Masuk ke direktori `backend/`.
2. Jalankan:
   ```bash
   cargo build --release
   cargo run
