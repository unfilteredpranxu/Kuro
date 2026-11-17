use dioxus::prelude::*;
use kuro_core::{State, validate};
use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/public/storage.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn load() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub async fn save(json: &str, expected: u32) -> Result<JsValue, JsValue>;
    pub fn today() -> String;
    pub fn uid() -> String;
    pub fn download(text: &str, name: &str);
    #[wasm_bindgen(catch)]
    pub async fn pick_backup() -> Result<JsValue, JsValue>;
    pub fn shortcuts();
    pub fn dialog_focus(open: bool);
    #[wasm_bindgen(catch)]
    pub fn save_draft(json: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    pub fn load_draft() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub fn clear_draft() -> Result<(), JsValue>;
}
#[derive(Clone, Copy, PartialEq)]
pub struct AppStore {
    pub state: Signal<State>,
    pub busy: Signal<bool>,
    pub ready: Signal<bool>,
    pub error: Signal<String>,
    pub message: Signal<String>,
}
pub fn describe(e: JsValue) -> String {
    js_sys::Reflect::get(&e, &"message".into())
        .ok()
        .and_then(|x| x.as_string())
        .or_else(|| e.as_string())
        .unwrap_or_else(|| "Browser storage unavailable. No changes were saved.".into())
}
pub fn commit(store: AppStore, change: impl FnOnce(&mut State) + 'static) {
    commit_then(store, change, || {});
}
pub fn commit_then(
    mut store: AppStore,
    change: impl FnOnce(&mut State) + 'static,
    after: impl FnOnce() + 'static,
) {
    if !*store.ready.read() || *store.busy.read() {
        return;
    }
    let expected = store.state.read().revision;
    let mut next = store.state.peek().clone();
    change(&mut next);
    if let Err(e) = validate(&next) {
        store.error.set(e);
        return;
    }
    let Some(revision) = expected.checked_add(1) else {
        store
            .error
            .set("Revision limit reached; export your data.".into());
        return;
    };
    next.revision = revision;
    let json = match serde_json::to_string(&next) {
        Ok(x) => x,
        Err(e) => {
            store.error.set(e.to_string());
            return;
        }
    };
    store.busy.set(true);
    store.message.set(String::new());
    dioxus::core::spawn_forever(async move {
        match save(&json, expected).await {
            Ok(_) => {
                store.state.set(next);
                store.error.set(String::new());
                store.message.set("Saved on this device".into());
                after();
            }
            Err(e) => store.error.set(describe(e)),
        }
        store.busy.set(false);
    });
}
