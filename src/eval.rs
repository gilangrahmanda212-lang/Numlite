use std::collections::HashMap;
use crate::ast::{BinOp, Expr, Stmt};
use crate::value::{Matrix, Value};
use crate::builtins::{call_builtin, elementwise, matrix_mul};

pub type Env = HashMap<String, Value>;

#[derive(Debug, PartialEq)]
pub enum ControlFlow {
    Normal,
    Break,
    Continue,
}

pub fn eval(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => Ok(Value::Scalar(*n)),
        Expr::String(s) => Ok(Value::Str(s.clone())),
        Expr::Var(name) => env.get(name).cloned().ok_or_else(|| format!("Variabel '{}' belum didefinisikan", name)),
        Expr::Not(inner) => {
            let v = eval(inner, env)?.as_scalar().unwrap_or(0.0);
            Ok(Value::Scalar(if v == 0.0 { 1.0 } else { 0.0 }))
        }
        Expr::Binary { op, left, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
            apply_binop(*op, &l, &r)
        }
        Expr::MatrixLit { rows } => eval_matrix_lit(rows, env),
        Expr::Transpose(inner) => {
            let v = eval(inner, env)?;
            Ok(Value::Matrix(transpose(&v.as_matrix())))
        }
        Expr::CellLit { elements } => {
            let mut vals = Vec::new();
            for el in elements { vals.push(eval(el, env)?); }
            Ok(Value::Cell(vals))
        }
        Expr::StructAccess { object, field } => {
            let obj_val = eval(object, env)?;
            match obj_val {
                Value::Struct(map) => map.get(field).cloned().ok_or_else(|| format!("Field '{}' tidak ditemukan", field)),
                _ => Err(format!("Objek bukan struct, tidak bisa mengakses field '{}'", field)),
            }
        }
        Expr::Call { name, args } => {
            let mut values = Vec::new();
            for arg in args { values.push(eval(arg, env)?); }
            
            if let Some(Value::Function(f)) = env.get(name) {
                if f.params.len() != values.len() {
                    return Err(format!("Fungsi '{}' butuh {} argumen, diberi {}", name, f
                    .params.len(), values.len()));
                }
                let mut local_env = env.clone();
                for (param, val) in f.params.iter().zip(values.iter()) {
                    local_env.insert(param.clone(), val.clone());
                }
                let mut captured = String::new();
                for stmt in &f.body {
                    exec_stmt(stmt, &mut local_env, &mut captured, false)?;
                }
                
                let mut return_val = None;
                for (key, val) in &local_env {
                    if !env.contains_key(key) && !f.params.contains(key) {
                        return_val = Some(val.clone());
                        break;
                    }
                }
                if return_val.is_none() {
                    return_val = local_env.get(name).or_else(|| local_env.get("ans")).cloned();
                }

                if let Some(v) = return_val { Ok(v) }
                else { Err(format!("Fungsi '{}' tidak mengembalikan nilai", name)) }
            } else {
                call_builtin(name, &values)
            }
        }
        Expr::Range { start, step, end } => {
            let start_val = eval(start, env)?.as_scalar().ok_or("Range: start harus scalar")?;
            let end_val = eval(end, env)?.as_scalar().ok_or("Range: end harus scalar")?;
            let step_val = if let Some(s) = step { eval(s, env)?.as_scalar().ok_or("Range: step harus scalar")? } else { 1.0 };
            if step_val == 0.0 { return Err("Range: step tidak boleh 0".to_string()); }
            let mut data = Vec::new();
            let mut current = start_val;
            if step_val > 0.0 {
                while current <= end_val + 1e-9 { data.push(current); current += step_val; }
            } else {
                while current >= end_val - 1e-9 { data.push(current); current += step_val; }
            }
            Ok(Value::Matrix(Matrix::new(1, data.len(), data)))
        }
        Expr::Index { target, index } => {
            let target_val = eval(target, env)?;
            let mat = target_val.as_matrix();
            let idx_mat = if let Expr::Call { name, .. } = &**index {
                if name == "get_end" {
                    Matrix::new(1, 1, vec![mat.data.len() as f64])
                } else { eval(index, env)?.as_matrix() }
            } else { eval(index, env)?.as_matrix() };

            let mut result_data = Vec::new();
            for &idx in &idx_mat.data {
                let i = (idx as usize).saturating_sub(1);
                if i < mat.data.len() { result_data.push(mat.data[i]); }
                else { return Err(format!("Index {} di luar batas array", idx)); }
            }
            Ok(Value::Matrix(Matrix::new(1, result_data.len(), result_data)))
        }
    }
}

fn eval_matrix_lit(rows: &[Vec<Expr>], env: &Env) -> Result<Value, String> {
    if rows.is_empty() { return Ok(Value::Matrix(Matrix::zeros(0, 0))); }
    let mut data = Vec::new();
    for row in rows {
        for cell in row {
            data.push(eval(cell, env)?.as_scalar().ok_or("Elemen harus scalar")?);
        }
    }
    Ok(Value::Matrix(Matrix::new(rows.len(), rows[0].len(), data)))
}

fn transpose(m: &Matrix) -> Matrix {
    let mut res = Matrix::zeros(m.cols, m.rows);
    for r in 0..m.rows {
        for c in 0..m.cols { res.set(c, r, m.get(r, c)); }
    }
    res
}

fn apply_binop(op: BinOp, l: &Value, r: &Value) -> Result<Value, String> {
    match op {
        BinOp::Add => elementwise(l, r, |a, b| Ok(a + b)),
        BinOp::Sub => elementwise(l, r, |a, b| Ok(a - b)),
        BinOp::EMul => elementwise(l, r, |a, b| Ok(a * b)),
        BinOp::EDiv => elementwise(l, r, |a, b| if b == 0.0 { Err("Div 0".into()) } else { Ok(a / b) }),
        BinOp::EPow => elementwise(l, r, |a, b| Ok(a.powf(b))),
        BinOp::Mul => matrix_mul(l, r),
        BinOp::Div => elementwise(l, r, |a, b| if b == 0.0 { Err("Div 0".into()) } else { Ok(a / b) }),
        BinOp::Pow => elementwise(l, r, |a, b| Ok(a.powf(b))),
        
        BinOp::Eq => elementwise(l, r, |a, b| Ok(if a == b { 1.0 } else { 0.0 })),
        BinOp::Neq => elementwise(l, r, |a, b| Ok(if a != b { 1.0 } else { 0.0 })),
        BinOp::Lt => elementwise(l, r, |a, b| Ok(if a < b { 1.0 } else { 0.0 })),
        BinOp::Gt => elementwise(l, r, |a, b| Ok(if a > b { 1.0 } else { 0.0 })),
        BinOp::Le => elementwise(l, r, |a, b| Ok(if a <= b { 1.0 } else { 0.0 })),
        BinOp::Ge => elementwise(l, r, |a, b| Ok(if a >= b { 1.0 } else { 0.0 })),
        
        BinOp::And => elementwise(l, r, |a, b| Ok(if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 })),
        BinOp::Or => elementwise(l, r, |a, b| Ok(if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 })),
    }
}

pub fn format_string(fmt: &str, args: &[Value]) -> String {
    let mut out = String::new();
    let mut arg_idx = 0;
    let mut chars = fmt.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut spec = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphabetic() { break; }
                spec.push(chars.next().unwrap());
            }
            if let Some(type_c) = chars.next() {
                if arg_idx < args.len() {
                    let val = &args[arg_idx];
                    let scalar = val.as_scalar().unwrap_or(0.0);
                    let mut handled = false;
                    
                    if type_c == 'd' || type_c == 'i' {
                        out.push_str(&format!("{}", scalar as i64));
                        handled = true;
                    } else if type_c == 'f' || type_c == 'g' || type_c == 'e' {
                        if let Some(stripped) = spec.strip_prefix('.') {
                            if let Ok(prec) = stripped.parse::<usize>() {
                                out.push_str(&format!("{:.*}", prec, scalar));
                                handled = true;
                            }
                        }
                        if !handled {
                            out.push_str(&format!("{}", scalar));
                            handled = true;
                        }
                    } else if type_c == 's' {
                        match val {
                            Value::Str(s) => out.push_str(s),
                            v => out.push_str(&format!("{}", v)),
                        }
                        handled = true;
                    }
                    
                    if !handled {
                        out.push('%');
                        out.push_str(&spec);
                        out.push(type_c);
                    }
                    arg_idx += 1;
                } else {
                    out.push('%');
                    out.push_str(&spec);
                    out.push(type_c);
                }
            } else {
                out.push('%');
            }
        } else if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some(other) => { out.push('\\'); out.push(other); },
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn exec_stmt(stmt: &Stmt, env: &mut Env, capture_output: &mut String, is_in_loop: bool) -> Result<ControlFlow, String> {
    match stmt {
        Stmt::Plot { .. } => Ok(ControlFlow::Normal),
        Stmt::Hold(_) => Ok(ControlFlow::Normal),
        
        Stmt::StructAssign { object, field, value } => {
            let val = eval(value, env)?;
            
            let mut map = match env.get(object) {
                Some(Value::Struct(m)) => m.clone(),
                None => HashMap::new(),
                _ => return Err(format!("Variabel '{}' sudah ada dan bukan struct", object)),
            };

            let parts: Vec<&str> = field.split('.').collect();
            
            fn insert_nested(map: &mut HashMap<String, Value>, parts: &[&str], val: Value) -> Result<(), String> {
                if parts.len() == 1 {
                    map.insert(parts[0].to_string(), val);
                    Ok(())
                } else {
                    let key = parts[0].to_string();
                    let next_val = map.entry(key).or_insert_with(|| Value::Struct(HashMap::new()));
                    match next_val {
                        Value::Struct(inner_map) => insert_nested(inner_map, &parts[1..], val),
                        _ => Err(format!("Field '{}' sudah ada dan bukan struct", parts[0])),
                    }
                }
            }

            if let Err(e) = insert_nested(&mut map, &parts, val) {
                return Err(e);
            }

            env.insert(object.clone(), Value::Struct(map));
            
            if !is_in_loop {
                capture_output.push_str(&format!("{}.{} = OK\n", object, field));
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::Expr(Expr::Call { name, args }) if name == "run" => {
            if args.len() != 1 { return Err("run: butuh 1 argumen".to_string()); }
            let filename_val = eval(&args[0], env)?;
            let filename = match filename_val {
                Value::Str(s) => s,
                _ => return Err("run: nama file harus string".to_string()),
            };
            let content = std::fs::read_to_string(&filename)
                .map_err(|e| format!("Gagal membaca file {}: {}", filename, e))?;
            
            let mut lexer = crate::lexer::Lexer::new(&content);
            let tokens = lexer.tokenize()?;
            let mut parser = crate::parser::Parser::new(tokens);

            while !parser.is_at_eof() {
                if let Ok(stmt) = parser.parse_stmt() {
                    exec_stmt(&stmt, env, capture_output, is_in_loop)?;
                } else { break; }
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::Expr(Expr::Call { name, .. }) if name == "subplot" || name == "figure" || name == "grid" || name == "clc" || name == "clear" || name == "close" => {
            Ok(ControlFlow::Normal)
        }

        Stmt::Expr(Expr::Call { name, args }) if name == "fprintf" => {
            if args.is_empty() { return Err("fprintf: butuh argumen".to_string()); }
            let mut values = Vec::new();
            for arg in args { values.push(eval(arg, env)?); }
            if let Some(Value::Str(fmt)) = values.get(0) {
                let formatted = format_string(fmt, &values[1..]);
                capture_output.push_str(&formatted);
                Ok(ControlFlow::Normal)
            } else {
                Err("fprintf: argumen pertama harus string format".to_string())
            }
        }
        
        Stmt::Expr(expr) => {
            if let Expr::Call { name, args } = expr {
                if name == "disp" && args.len() == 1 {
                    let val = eval(&args[0], env)?;
                    match val {
                        Value::Str(s) => capture_output.push_str(&format!("{}\n", s)),
                        Value::Scalar(n) => capture_output.push_str(&format!("{}\n", n)),
                        v => capture_output.push_str(&format!("{}\n", v)),
                    }
                    return Ok(ControlFlow::Normal);
                }
            }

            let val = eval(expr, env)?;
            match &val {
                Value::Scalar(n) => capture_output.push_str(&format!("ans = {}\n", n)),
                Value::Complex(_) => capture_output.push_str(&format!("ans = {}\n", val)),
                _ => capture_output.push_str(&format!("ans =\n{}\n", val)),
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::Assign { name, value } => {
            let v = eval(value, env)?;
            if !is_in_loop {
                match &v {
                    Value::Scalar(n) => capture_output.push_str(&format!("{} = {}\n", name, n)),
                    Value::Complex(_) => capture_output.push_str(&format!("{} = {}\n", name, v)),
                    _ => capture_output.push_str(&format!("{} =\n{}\n", name, v)),
                }
            }
            env.insert(name.clone(), v);
            Ok(ControlFlow::Normal)
        }
        
        Stmt::For { var, iter, body } => {
            let iter_val = eval(iter, env)?;
            for val in iter_val.as_matrix().data {
                env.insert(var.clone(), Value::Scalar(val));
                let mut should_break = false;
                for s in body { 
                    match exec_stmt(s, env, capture_output, true)? {
                        ControlFlow::Break => { should_break = true; break; } 
                        ControlFlow::Continue => break, 
                        ControlFlow::Normal => {}
                    }
                }
                if should_break { break; }
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::While { cond, body } => {
            loop {
                let cond_val = eval(cond, env)?;
                let is_true = cond_val.as_scalar().unwrap_or(0.0) != 0.0;
                if !is_true { break; }
                let mut should_break = false;
                for s in body { 
                    match exec_stmt(s, env, capture_output, true)? {
                        ControlFlow::Break => { should_break = true; break; }
                        ControlFlow::Continue => { break; }
                        ControlFlow::Normal => {}
                    }
                }
                if should_break { break; }
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::If { cond, then_body, else_body } => {
            let cond_val = eval(cond, env)?;
            let is_true = cond_val.as_scalar().unwrap_or(0.0) != 0.0;
            if is_true {
                for s in then_body { 
                    let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                    if cf != ControlFlow::Normal { return Ok(cf); }
                }
            } else if let Some(eb) = else_body {
                for s in eb { 
                    let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                    if cf != ControlFlow::Normal { return Ok(cf); }
                }
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::Switch { expr, cases, otherwise } => {
            let val = eval(expr, env)?;
            let mut matched = false;
            for (case_expr, body) in cases {
                let case_val = eval(case_expr, env)?;
                let is_match = match (&val, &case_val) {
                    (Value::Scalar(a), Value::Scalar(b)) => (a - b).abs() < 1e-9,
                    (Value::Str(a), Value::Str(b)) => a == b,
                    _ => false,
                };
                if is_match {
                    for s in body {
                        let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                        if cf != ControlFlow::Normal { return Ok(cf); }
                    }
                    matched = true;
                    break;
                }
            }
            if !matched {
                if let Some(eb) = otherwise {
                    for s in eb {
                        let cf = exec_stmt(s, env, capture_output, is_in_loop)?;
                        if cf != ControlFlow::Normal { return Ok(cf); }
                    }
                }
            }
            Ok(ControlFlow::Normal)
        }
        
        Stmt::Break => Ok(ControlFlow::Break),
        Stmt::Continue => Ok(ControlFlow::Continue),
        
        Stmt::FunctionDef(f) => {
            env.insert(f.name.clone(), Value::Function(f.clone()));
            Ok(ControlFlow::Normal)
        }
    }
}
