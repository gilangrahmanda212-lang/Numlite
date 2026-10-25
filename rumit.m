% ==========================================
% UJI SLICING & DSP (NUM LITE)
% ==========================================

% 1. Membuat Vektor Waktu
t = 0:0.1:5

% 2. Membuat Sinyal Asli (Sinus 2 Hz)
sinyal_asli = sin(2 * 3.14159 * 2 * t)

% 3. Menambahkan Noise
noise = rand(1, length(t)) * 0.5 - 0.25
sinyal_noise = sinyal_asli + noise

% 4. Filtering (Konvolusi)
filter = [0.2, 0.2, 0.2, 0.2, 0.2]
sinyal_halus = conv(sinyal_noise, filter)
t_baru = 1:length(sinyal_halus)

% 5. UJI SLICING: Mengambil potongan sinyal
% Kita ingin mengambil sinyal halus sepanjang vektor t asli
panjang_asli = length(t)
sinyal_potong = sinyal_halus(1:panjang_asli)

% 6. UJI LOGIKA: Mengecek nilai maksimum
nilai_maks = max(sinyal_potong)
if nilai_maks > 0.8
    disp('Peringatan: Sinyal terlalu tinggi!')
else
    disp('Sinyal dalam batas aman.')
end

% 7. Menampilkan Grafik
subplot(3,1,1)
plot(t, sinyal_noise)
title('Sinyal Asli + Noise')

subplot(3,1,2)
plot(t_baru, sinyal_halus)
title('Sinyal Setelah Konvolusi')

subplot(3,1,3)
plot(t, sinyal_potong)
title('Hasil Slicing (Dipangkas)')
