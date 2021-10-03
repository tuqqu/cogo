use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use cogo_compiler::{compile, ErrorHandler};
use cogo_vm::io::StdStreamProvider;
use cogo_vm::{CUnitFrame, Vm};

struct TestErrorHandler(Vec<String>);

impl ErrorHandler for TestErrorHandler {
    fn on_error(&mut self, errs: &[Box<dyn Error>]) {
        for err in errs {
            self.0.push(err.to_string());
        }
    }
}

impl TestErrorHandler {
    fn new() -> Self {
        Self(vec![])
    }

    fn errs(&self) -> &[String] {
        &self.0
    }
}

pub fn compare_stderr_output(program: &str, expected_stderr: &str) {
    let stdout = Rc::new(RefCell::new(Vec::<u8>::new()));
    let stderr = Rc::new(RefCell::new(Vec::<u8>::new()));

    let vecout = Rc::clone(&stdout);
    let vecerr = Rc::clone(&stderr);

    let stream_provider = StdStreamProvider::new(Some((Some(stdout), Some(stderr), None)));

    let mut err_handler = TestErrorHandler::new();
    let cunit = compile(program, &mut err_handler);
    let frame = CUnitFrame::new(cunit);

    assert!(err_handler.errs().is_empty());

    let mut vm = Vm::new(Some(Box::new(stream_provider)), frame);
    let res = vm.run();

    assert!(res.is_ok(), "{}", res.err().unwrap().to_string());

    let _out = &*vecout.borrow();
    let _out = String::from_utf8_lossy(_out);

    let err = &*vecerr.borrow();
    let err = String::from_utf8_lossy(err);

    assert_eq!(err, expected_stderr);
}
