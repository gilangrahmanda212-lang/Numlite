disp('=== [2] KONTROL ALUR, LOGIKA, & STATISTIKA ===');
% Kategori 5: Logika
is_valid = 1;
is_error = 0;
if is_valid && ~is_error
    disp('Kondisi logika: VALID');
end

% Kategori 4: Control Flow (For & While)
disp('-- Perulangan --');
total_for = 0;
for i = 1:5
    total_for = total_for + i;
end
fprintf('Total angka 1 sampai 5 = %d\n', total_for);

counter = 0;
while counter < 3
    counter = counter + 1;
end
fprintf('While loop berhenti di counter = %d\n', counter);

% Kategori 4: Fungsi Buatan Sendiri
disp('-- Fungsi Kustom --');
function hasil = kuadrat(x)
    hasil = x * x;
end

kuadrat_5 = kuadrat(5);
fprintf('Kuadrat dari 5 = %d\n', kuadrat_5);

% Kategori 6: Statistika
disp('-- Statistika --');
data = [10, 20, 30, 40, 50];
rata2 = mean(data);
maks = max(data);
mins = min(data);
total = sum(data);

fprintf('Data: [10, 20, 30, 40, 50]\n');
fprintf('Mean: %.1f | Max: %.1f | Min: %.1f | Sum: %.1f\n', rata2, maks, mins, total);