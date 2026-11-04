% =======================================================
% UJI FITUR BARU: LOGIKA, STATISTIKA, & SINYAL
% =======================================================

% --- 1. UJI OPERATOR LOGIKA (&&, ||, ~) ---
disp('--- Uji Logika ---')
% && (AND)
logika_and1 = (1 && 1)   % Harus 1
logika_and2 = (1 && 0)   % Harus 0

% || (OR)
logika_or1 = (1 || 0)    % Harus 1
logika_or2 = (0 || 0)    % Harus 0

% ~ (NOT)
logika_not1 = (~0)       % Harus 1
logika_not2 = (~5)       % Harus 0

% Kombinasi Logika & Perbandingan
kombinasi = (10 > 5) && (3 < 2) % Harus 0 (Karena 3<2 salah)

% --- 2. UJI STATISTIKA (min & std) ---
disp('--- Uji Statistika ---')
data = [4, 2, 9, 1, 7]

nilai_min = min(data)    % Harus 1
nilai_std = std(data)    % Standar deviasi data

% --- 3. UJI PEMROSESAN SINYAL ---
disp('--- Uji Sinyal ---')

% A. Hamming Window (n=5)
% Rumus: 0.54 - 0.46 * cos(2*pi*k / (n-1))
win_hamming = hamming(5) 

% B. Cross-Correlation (xcorr)
% Sinyal x dan y sama-sama [1, 2, 3]
sig_x = [1, 2, 3]
sig_y = [1, 2, 3]
hasil_xcorr = xcorr(sig_x, sig_y)

% C. Filter FIR (Moving Average 2 titik)
% b = [1, 1] (koefisien filter), a = [1] (penyebut)
% x = [1, 2, 3, 4] (sinyal input)
b = [1, 1]
a = [1]
sinyal_in = [1, 2, 3, 4]
sinyal_out = filter(b, a, sinyal_in)