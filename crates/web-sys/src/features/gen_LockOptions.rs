#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LockOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LockOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type LockOptions;
    #[wasm_bindgen(method, setter = "ifAvailable")]
    fn if_available_shim(this: &LockOptions, val: bool);
    #[cfg(feature = "LockMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn mode_shim(this: &LockOptions, val: LockMode);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn signal_shim(this: &LockOptions, val: &AbortSignal);
    #[wasm_bindgen(method, setter = "steal")]
    fn steal_shim(this: &LockOptions, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl LockOptions {
    #[doc = "Construct a new `LockOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `ifAvailable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn if_available(&mut self, val: bool) -> &mut Self {
        self.if_available_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockMode`, `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mode(&mut self, val: LockMode) -> &mut Self {
        self.mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.signal_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `steal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn steal(&mut self, val: bool) -> &mut Self {
        self.steal_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for LockOptions {
    fn default() -> Self {
        Self::new()
    }
}
