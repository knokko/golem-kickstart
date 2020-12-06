#![cfg(target_arch = "wasm32")]
// This file (lib.rs) is only needed when you want to run it as web application

mod cross_platform;

use blinds::*;
use golem::{
    Context,
    GolemError
};
use wasm_bindgen::prelude::*;

// The web application loop, powered by the blinds crate
async fn app(
    window: Window,
    events: EventStream,
) -> Result<(), GolemError> {

    // Create a webgl context from 'glow', GL On Whatever
    let ctx = &Context::from_glow(glow::Context::from_webgl1_context(
        window.webgl_context()
    ))?;

    // Let the cross platform code do the rest of the work (to improve code reuse)
    cross_platform::start(window, ctx, events).await
}

// Run our web application!
#[wasm_bindgen(start)]
pub fn main() {
    run(Settings::default(), |window, events| async move {
        app(window, events).await.unwrap()
    });
}
