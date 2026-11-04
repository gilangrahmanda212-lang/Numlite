% =======================================================
% UJI KOMPREHENSIF SEMUA FITUR NUMLITE
% =======================================================

% --- 1. UJI FUNGSI BUATAN SENDIRI ---
function y = kuadrat(x)
    y = x * x
end

function res = cek_genap(x)
    if mod(x, 2) == 0
        res = 1
    else
        res = 0
    end
end

function total = jumlah_hingga(n)
    total = 0
    i = 1
    while i <= n
        total = total + i
        i = i + 1
    end
end

% Memanggil Fungsi
hasil_kuadrat = kuadrat(5)
cek_angka = cek_genap(4)
total_while = jumlah_hingga(5)

% --- 2. UJI KONTROL ALUR (SWITCH, FOR, BREAK, CONTINUE) ---
x_switch = 2
switch x_switch
    case 1
        hasil_switch = 10
    case 2
        hasil_switch = 20
    otherwise
        hasil_switch = 99
end

total_break = 0
for i = 1:10
    if i == 4
        break
    end
    total_break = total_break + i
end

total_continue = 0
for i = 1:5
    if i == 3
        continue
    end
    total_continue = total_continue + i
end

% --- 3. UJI MATRIKS & ALJABAR LINEAR ---
A = [1, 2; 3, 4]
B = [5, 6; 7, 8]

% Konkatenasi (Gabung)
C_gabung = [A, B]
D_gabung = [A; B]

% Aljabar Linear
inv_A = inv(A)
det_A = det(A)
trace_A = trace(A)
rank_A = rank(A)

% Manipulasi Matriks
M = [1, 2, 3, 4, 5, 6]
M_reshape = reshape(M, 2, 3)
M_diag = diag(A)
M_repmat = repmat(A, 2, 2)

% --- 4. UJI BILANGAN KOMPLEKS & MATEMATIKA ---
p = pi
z = complex(3, 4)
mag_z = abs(z)
k_z = conj(z)

% Uji Pembulatan
r = round(3.7)
m = mod(-10, 3)

% --- 5. UJI SLICING & END ---
v = [10, 20, 30, 40, 50]
v_potong = v(2:4)
v_terakhir = v(end)