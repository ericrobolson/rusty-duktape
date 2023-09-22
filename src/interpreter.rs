// Inspired by: https://stackoverflow.com/questions/38315383/whats-the-rust-idiom-to-define-a-field-pointing-to-a-c-opaque-pointer
#[repr(C)]
pub struct duk_context {
    _private: [u8; 0],
}

#[link(name = "c.lib", kind = "static")]
extern "C" {
    fn duk_create_heap_default() -> *mut duk_context;
    fn duk_eval_string(ctx: *mut duk_context, message: *const libc::c_char);
    fn duk_get_int(ctx: *mut duk_context, idx: i32) -> i32;
    fn duk_pop(ctx: *mut duk_context);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    UnableToInitContext,
}

pub struct Interpreter {
    ctx: *mut duk_context,
}

impl Interpreter {
    pub fn new() -> Result<Self, Error> {
        let ctx = unsafe {
            let ctx = duk_create_heap_default();
            if ctx.is_null() {
                return Err(Error::UnableToInitContext);
            }

            ctx
        };

        Ok(Self { ctx })
    }

    pub fn interpret<'a>(&mut self, code: &'a str) {
        let code = std::ffi::CString::new(code.as_bytes()).unwrap();

        unsafe {
            duk_eval_string(self.ctx, code.as_ptr());
        }
    }

    pub fn pop(&mut self) {
        unsafe { duk_pop(self.ctx) }
    }

    pub fn pop_i32(&mut self) -> i32 {
        unsafe { duk_get_int(self.ctx, -1) }
    }
}
