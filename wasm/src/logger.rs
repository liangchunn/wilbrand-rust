use std::{cell::RefCell, sync::Once};

use js_sys::Function;
use log::{Level, LevelFilter, Metadata, Record};
use wasm_bindgen::prelude::*;

thread_local! {
    static LOG_BUFFER: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}
static LOGGER_HOOK: Once = Once::new();
static LOGGER: Logger = Logger;

// logging stuff back to js via callback (poll via flush_logs)
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            LOG_BUFFER.with(|buf| {
                buf.borrow_mut()
                    .push(format!("[{}] {}", record.level(), record.args()));
            });
        }
    }

    fn flush(&self) {}
}

#[wasm_bindgen]
pub fn init_logger() {
    console_error_panic_hook::set_once();
    LOGGER_HOOK.call_once(|| {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(LevelFilter::Trace);
    });
}

#[wasm_bindgen]
pub fn flush_logs(callback: &Function) {
    LOG_BUFFER.with(|buf| {
        for msg in buf.borrow_mut().drain(..) {
            let _ = callback.call1(&JsValue::NULL, &JsValue::from_str(&msg));
        }
    });
}
