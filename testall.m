% =======================================================
% SKRIP UJI COBA SELURUH FITUR NUMLITE
% =======================================================

% --- 0. DEFINISI FUNGSI (HARUS DI ATAS) ---
function y = hitung_kuadrat(x)
    y = x * x
end

function res = cek_genap(x)
    if mod(x, 2) == 0
        res = 1
    else
        res = 0
    end
end

% --- 1. UJI MATEMATIKA DASAR & MATRIKS ---
disp('=== 1. UJI MATEMATIKA DASAR & MATRIKS ===');
a = 5 + 3 * 2^2; % Prioritas operator
b = mod(10, 3);
c = round(3.7);
disp('Nilai a, b, c:');
disp([a, b, c]);

A = [1, 2; 3, 4]
B = [5, 6; 7, 8]
disp('Hasil A * B:');
disp(A * B);
disp('Determinan A:');
disp(det(A));
disp('Invers A:');
disp(inv(A));

% --- 2. UJI KONTROL ALUR (IF, FOR, WHILE, SWITCH) ---
disp('=== 2. UJI CONTROL FLOW ===');
x = 10;
if x > 5
    disp('If: x lebih besar dari 5');
end

total = 0;
for i = 1:5
    total = total + i;
end
fprintf('For: Hasil penjumlahan 1 sampai 5 = %d\n', total);

hitung = 3;
while hitung > 0
    disp('While: Hitung mundur...');
    hitung = hitung - 1;
end

warna = 2;
switch warna
    case 1
        disp('Switch: Merah');
    case 2
        disp('Switch: Biru');
    otherwise
        disp('Switch: Warna lain');
end

% --- 3. UJI FUNGSI BUATAN SENDIRI ---
disp('=== 3. UJI FUNGSI BUATAN SENDIRI ===');
hasil_kuadrat = hitung_kuadrat(5)
status_genap = cek_genap(4)

% --- 4. UJI TRIGONOMETRI & LOGIKA ---
disp('=== 4. UJI TRIGONOMETRI & LOGIKA ===');
sudut = pi / 2;
s = sin(sudut);
disp('Sin(90 derajat) =');
disp(s);

is_valid = 1;
is_error = 0;
if is_valid && ~is_error
    disp('Logika: Kondisi VALID');
end

% --- 5. UJI STATISTIKA ---
disp('=== 5. UJI STATISTIKA ===');
data = [10, 20, 30, 40, 50];
rata2 = mean(data);
std_dev = std(data);
nilai_max = max(data);
fprintf('Data: [10, 20, 30, 40, 50]\n');
fprintf('Mean: %.1f | Std Dev: %.2f | Max: %.1f\n', rata2, std_dev, nilai_max);

% --- 6. UJI PEMROSESAN SINYAL ---
disp('=== 6. UJI PEMROSESAN SINYAL ===');
sig_x = [1, 2, 3];
sig_y = [1, 2, 3];
hasil_conv = conv(sig_x, sig_y);
disp('Hasil Konvolusi:');
disp(hasil_conv);

win = hamming(5);
disp('Window Hamming (5 titik):');
disp(win);

% --- 7. UJI STRING & FORMAT TEKS ---
disp('=== 7. UJI STRING & FORMAT TEKS ===');
nama = 'NumLite';
versi = 1.0;
teks_gabung = strcat('Selamat datang di ', nama);
disp(teks_gabung);

angka_str = num2str(100);
disp(str2num(angka_str) + 50);
fprintf('Format Float: %.3f\n', pi);

% --- 8. UJI STRUCT & CELL ARRAY ---
disp('=== 8. UJI STRUCT & CELL ARRAY ===');
mhs.nama = 'Gilang';
mhs.umur = 21;
mhs.alamat.kota = 'Jakarta'; % Nested Struct

disp('Nama Mahasiswa:');
disp(mhs.nama);
disp('Kota Mahasiswa:');
disp(mhs.alamat.kota);

data_cell = {1, 'Halo', [1, 2; 3, 4], 3.14};
disp('Isi Cell Array:');
disp(data_cell);

% --- 9. UJI VISUALISASI (GRAFIK) ---
disp('=== 9. UJI VISUALISASI (CEK PANEL KANAN) ===');

% Grafik 1: Plot(x,y) dengan judul & label
x_plot = 1:5;
y_plot = [2, 5, 3, 8, 1];

title('Grafik Garis & Batang')
xlabel('Waktu (s)')
ylabel('Amplitudo')
plot(x_plot, y_plot)

% Tahan grafik untuk menambahkan Stem
hold on
stem(x_plot, y_plot)
hold off

% Grafik 2: Plot 1 Argumen (otomatis sumbu X)
y_otomatis = [10, 20, 15, 30, 25];
title('Grafik 1 Argumen')
xlabel('Indeks')
ylabel('Nilai')
plot(y_otomatis)

disp('=== SELURUH UJI COBA SELESAI ===');