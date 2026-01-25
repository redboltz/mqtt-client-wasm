//! Platform abstraction layer
//!
//! This module provides platform-agnostic APIs for timers, logging, and time.
//! These work in both browser and Node.js environments by using global JavaScript functions.

// ============================================================================
// WASM32 Platform Functions
// ============================================================================

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        /// Set a timeout using global setTimeout (works in browser and Node.js)
        #[wasm_bindgen(js_name = setTimeout)]
        fn set_timeout_internal(closure: &js_sys::Function, millis: i32) -> i32;

        /// Clear a timeout using global clearTimeout (works in browser and Node.js)
        #[wasm_bindgen(js_name = clearTimeout)]
        pub fn clear_timeout(id: i32);
    }

    /// Set a timeout - wrapper that accepts Closure<dyn Fn()>
    pub fn set_timeout(closure: &Closure<dyn Fn()>, millis: i32) -> i32 {
        use wasm_bindgen::JsCast;
        set_timeout_internal(closure.as_ref().unchecked_ref(), millis)
    }

    #[wasm_bindgen]
    extern "C" {
        /// Get current time in milliseconds (Date.now())
        #[wasm_bindgen(js_namespace = Date, js_name = now)]
        pub fn date_now() -> f64;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = log)]
        pub fn console_log(s: &str);

        #[wasm_bindgen(js_namespace = console, js_name = warn)]
        pub fn console_warn(s: &str);

        #[wasm_bindgen(js_namespace = console, js_name = error)]
        pub fn console_error(s: &str);
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm32::*;

// ============================================================================
// Logging Macros (work on all platforms)
// ============================================================================

/// Log a message
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        $crate::platform::console_log(&format!($($arg)*));
        #[cfg(not(target_arch = "wasm32"))]
        println!($($arg)*);
    };
}

/// Log a warning
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        $crate::platform::console_warn(&format!($($arg)*));
        #[cfg(not(target_arch = "wasm32"))]
        eprintln!("WARN: {}", format!($($arg)*));
    };
}

/// Log an error
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        $crate::platform::console_error(&format!($($arg)*));
        #[cfg(not(target_arch = "wasm32"))]
        eprintln!("ERROR: {}", format!($($arg)*));
    };
}
