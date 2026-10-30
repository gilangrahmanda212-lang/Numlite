disp('=== [3] SINYAL & VISUALISASI ===');
% Kategori 7: Pemrosesan Sinyal
disp('-- Sinyal --');
x_sinyal = [1, 2, 3];
h_sinyal = [1, 1];
konvolusi = conv(x_sinyal, h_sinyal);
disp('Hasil Konvolusi [1, 2, 3] * [1, 1] =');
disp(konvolusi);

% Kategori 8: Visualisasi Grafik
disp('-- Grafik (Cek Panel Kanan!) --');
waktu = [1, 2, 3, 4, 5];
amplitudo = [2, 5, 3, 8, 1];

% 1. Buat grafik garis (Plot)
title('Sinyal Uji Coba')
xlabel('Waktu (s)')
ylabel('Amplitudo')
plot(waktu, amplitudo)

% 2. Tahan grafik, tambahkan Stem (Batang)
hold on

stem(waktu, amplitudo)

% 3. Lepas tahanan, buat grafik baru (Kuadrat)
hold off

x_baru = linspace(0, 5, 5); % Menghasilkan [0, 1.25, 2.5, 3.75, 5]
y_baru = [0, 1, 4, 9, 16];
title('Grafik Fungsi Kuadrat')
plot(x_baru, y_baru)

disp('=== SELURUH UJI COBA SELESAI ===');