use cpython::{Python, PyDict, PyResult};

fn main() {
    let gil = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

fn hello(py: Python) -> PyResult<()> {
    let locals = PyDict::new(py);
    locals.set_item(py, "inference", py.import("inference")?)?;
    let inference: String = py.eval("inference.call('test_message')", None, Some(&locals))?.extract(py)?;

    println!("Return: {}", inference);
    Ok(())
}
