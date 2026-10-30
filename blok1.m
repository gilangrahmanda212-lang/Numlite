disp('=== [1] ARITMETIKA, MATRIKS, & TRIGONOMETRI ===');
% Kategori 1: Aritmetika Dasar
a = 5 + 3 * 2^2;
b = mod(10, 3);
fprintf('Hasil a = %d, Hasil b = %d\n', a, b);

% Kategori 3: Trigonometri & Matematika
sudut = pi / 2;
s = sin(sudut);
akar = sqrt(16);
fprintf('Sin(90) = %.1f, Akar(16) = %.1f\n', s, akar);

% Kategori 2: Matriks & Aljabar Linear
disp('-- Operasi Matriks --');
M1 = [1, 2; 3, 4];
M2 = [5, 6; 7, 8];
M_kali = M1 * M2;
M_inv = inv(M1);
M_det = det(M1);
M_trans = M1';

disp('M1 * M2 =');
disp(M_kali);
fprintf('Determinan M1 = %.1f\n', M_det);

% Kategori 9: Manipulasi String
disp('-- Manipulasi String --');
teks_angka = num2str(100);
teks_gabung = strcat('Skor: ', teks_angka);
disp(teks_gabung);

angka_kembali = str2num('50') + 10;
fprintf('Hasil str2num + 10 = %d\n', angka_kembali);