use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


struct ViewModel {
    count: i32,
    target: web_sys::Element,
}
impl ViewModel {
    fn increment(&mut self) {
        self.count = self.count + 1;
        self.target.set_inner_html(&self.count.to_string());
    }
}

#[wasm_bindgen]
pub fn start() {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");
    
    let count_display = document.get_element_by_id("currentCount").expect("could not find element with id `currentCount`");
    let mut view_model = ViewModel {
        count: 0,
        target: count_display,
    };
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        view_model.increment()
    }) as Box<dyn FnMut(_)>);

    document.get_element_by_id("incrementButton")
        .expect("could not find element with id `incrementButton`")
        .dyn_ref::<HtmlElement>()
        .expect("incrementButton should be an `HtmlElement`")
        .set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
