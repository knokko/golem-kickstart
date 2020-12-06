#![cfg(not(target_arch = "wasm32"))]
// This file (main.rs) is only used when running this as a desktop application

mod cross_platform;
mod choose_settings;

use blinds::*;
use golem::{
    Context,
    GolemError
};

// The desktop application loop, powered by the blinds crate
async fn app(
    window: Window,
    events: EventStream,
) -> Result<(), GolemError> {

    // Create a native context from 'glow', GL On Whatever
    let ctx = &Context::from_glow(glow::Context::from_loader_function(|s| {
        window.get_proc_address(s) as *const _
    }))?;

    // Let the cross-platform code do the rest
    cross_platform::start(&window, ctx, events, || false).await
}

// Run our desktop application!
pub fn main() {
    run(choose_settings::create(), |window, events| async move {
        app(window, events).await.unwrap()
    });
}
