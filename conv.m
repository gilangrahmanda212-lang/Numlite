% 1. Buat sinyal asli (input)
x = [1, 2, 3, 4, 5]

% 2. Buat respons impuls (misal: filter rata-rata 3 titik)
h = [1/3, 1/3, 1/3]

% 3. Lakukan konvolusi
y = conv(x, h)

% 4. Buat sumbu X tiruan untuk grafik
% Panjang hasil konvolusi adalah panjang x + panjang h - 1
panjang_y = length(y)
t_x = 1:panjang_y

% 5. Tampilkan di GUI!
plot(t_x, y)
