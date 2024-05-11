use pyo3::prelude::*;

fn _fast_find_fibonacci_num(num: u32) -> u32 {
    if num <= 1 {
        return num;
    }
    let mut a = 0;
    let mut b = 1;
    for _i in 2..=num {
        std::mem::swap(&mut a, &mut b);
        b = a + b;
    }
    b
}

fn _recursive_find_fibonacci_num(num: u32) -> u32 {
    if num <= 1 {
        return num;
    }
    _recursive_find_fibonacci_num(num - 1) + _recursive_find_fibonacci_num(num - 2)
}

#[pyfunction]
fn fast_find_fibonacci_num(num: u32) -> u32 {
    return _fast_find_fibonacci_num(num);
}


#[pyfunction]
fn recursive_find_fibonacci_num(num: u32) -> u32 {
    return _recursive_find_fibonacci_num(num);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fast_find_fibonacci_num, m)?)?;
    m.add_function(wrap_pyfunction!(recursive_find_fibonacci_num, m)?)?;
    Ok(())
}
