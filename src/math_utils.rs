use crate::value::Matrix;

pub fn fft_recursive(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    if n <= 1 { return; }
    let mut re_even = vec![0.0; n / 2];
    let mut im_even = vec![0.0; n / 2];
    let mut re_odd = vec![0.0; n / 2];
    let mut im_odd = vec![0.0; n / 2];
    for i in 0..n / 2 {
        re_even[i] = re[2 * i]; im_even[i] = im[2 * i];
        re_odd[i] = re[2 * i + 1]; im_odd[i] = im[2 * i + 1];
    }
    fft_recursive(&mut re_even, &mut im_even);
    fft_recursive(&mut re_odd, &mut im_odd);
    for i in 0..n / 2 {
        let angle = -2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        let wr = angle.cos(); let wi = angle.sin();
        let tr = wr * re_odd[i] - wi * im_odd[i];
        let ti = wr * im_odd[i] + wi * re_odd[i];
        re[i] = re_even[i] + tr; im[i] = im_even[i] + ti;
        re[i + n / 2] = re_even[i] - tr; im[i + n / 2] = im_even[i] - ti;
    }
}

pub fn invert_matrix(m: &Matrix) -> Result<Matrix, String> {
    if m.rows != m.cols { return Err("inv: matriks harus persegi".to_string()); }
    let n = m.rows;
    let mut aug = Matrix::zeros(n, n * 2);
    for r in 0..n { for c in 0..n { aug.set(r, c, m.get(r, c)); aug.set(r, c + n, if r == c { 1.0 } else { 0.0 }); } }
    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n { if aug.get(k, i).abs() > aug.get(max_row, i).abs() { max_row = k; } }
        if aug.get(max_row, i).abs() < 1e-9 { return Err("inv: matriks singular".to_string()); }
        if i != max_row { for j in 0..(n * 2) { let tmp = aug.get(i, j); aug.set(i, j, aug.get(max_row, j)); aug.set(max_row, j, tmp); } }
        let pivot = aug.get(i, i);
        for j in 0..(n * 2) { aug.set(i, j, aug.get(i, j) / pivot); }
        for k in 0..n { if k != i { let factor = aug.get(k, i); for j in 0..(n * 2) { aug.set(k, j, aug.get(k, j) - factor * aug.get(i, j)); } } }
    }
    let mut inv = Matrix::zeros(n, n);
    for r in 0..n { for c in 0..n { inv.set(r, c, aug.get(r, c + n)); } }
    Ok(inv)
}

pub fn randn_scalar() -> f64 {
    let u1: f64 = rand::random::<f64>().max(f64::EPSILON);
    let u2: f64 = rand::random::<f64>();
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

pub fn determinant(m: &Matrix) -> Result<f64, String> {
    let n = m.rows;
    if n == 1 { return Ok(m.get(0, 0)); }
    if n == 2 { return Ok(m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0)); }
    let mut temp = m.clone(); let mut det = 1.0;
    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n { if temp.get(k, i).abs() > temp.get(max_row, i).abs() { max_row = k; } }
        if temp.get(max_row, i).abs() < 1e-12 { return Ok(0.0); }
        if i != max_row { for j in 0..n { let tmp = temp.get(i, j); temp.set(i, j, temp.get(max_row, j)); temp.set(max_row, j, tmp); } det = -det; }
        let pivot = temp.get(i, i); det *= pivot;
        for k in i + 1..n { let factor = temp.get(k, i) / pivot; for j in i..n { temp.set(k, j, temp.get(k, j) - factor * temp.get(i, j)); } }
    }
    Ok(det)
}

pub fn matrix_rank(m: &Matrix) -> f64 {
    let mut temp = m.clone(); let mut rank = 0; let rows = temp.rows; let cols = temp.cols; let mut row = 0;
    for col in 0..cols {
        if row >= rows { break; }
        let mut max_row = row;
        for k in row + 1..rows { if temp.get(k, col).abs() > temp.get(max_row, col).abs() { max_row = k; } }
        if temp.get(max_row, col).abs() > 1e-9 {
            if row != max_row { for j in 0..cols { let tmp = temp.get(row, j); temp.set(row, j, temp.get(max_row, j)); temp.set(max_row, j, tmp); } }
            let pivot = temp.get(row, col);
            for k in row + 1..rows { let factor = temp.get(k, col) / pivot; for j in col..cols { temp.set(k, j, temp.get(k, j) - factor * temp.get(row, j)); } }
            rank += 1; row += 1;
        }
    }
    rank as f64
}

pub fn power_iteration(m: &Matrix, max_iter: usize) -> Result<f64, String> {
    let n = m.rows; let mut v = vec![1.0; n]; let mut eigenvalue = 0.0;
    for _ in 0..max_iter {
        let mut new_v = vec![0.0; n];
        for i in 0..n { for j in 0..n { new_v[i] += m.get(i, j) * v[j]; } }
        let mut max_val = 0.0;
        for &val in &new_v { if val.abs() > max_val { max_val = val.abs(); } }
        if max_val < 1e-12 { break; }
        for i in 0..n { new_v[i] /= max_val; }
        let mut dot_new_v = 0.0; let mut dot_v = 0.0;
        for i in 0..n { dot_new_v += new_v[i] * v[i]; dot_v += v[i] * v[i]; }
        eigenvalue = dot_new_v / dot_v; v = new_v;
    }
    Ok(eigenvalue)
}
