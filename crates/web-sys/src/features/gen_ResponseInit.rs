#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResponseInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResponseInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub type ResponseInit;
    #[wasm_bindgen(method, setter = "headers")]
    fn headers_shim(this: &ResponseInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "status")]
    fn status_shim(this: &ResponseInit, val: u16);
    #[wasm_bindgen(method, setter = "statusText")]
    fn status_text_shim(this: &ResponseInit, val: &str);
}
impl ResponseInit {
    #[doc = "Construct a new `ResponseInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.headers_shim(val);
        self
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status(&mut self, val: u16) -> &mut Self {
        self.status_shim(val);
        self
    }
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status_text(&mut self, val: &str) -> &mut Self {
        self.status_text_shim(val);
        self
    }
}
impl Default for ResponseInit {
    fn default() -> Self {
        Self::new()
    }
}
