//! Abstractions for interacting with the `state-context` extension.

use anyhow::Result;
use clap_sys::ext::state_context::{
    CLAP_EXT_STATE_CONTEXT, clap_plugin_state_context, clap_plugin_state_context_type,
};
use std::ffi::CStr;
use std::ptr::NonNull;

use super::state::{InputStream, OutputStream};
use super::Extension;
use crate::plugin::instance::Plugin;
use crate::util::unsafe_clap_call;

/// Abstraction for the `state-context` extension.
#[derive(Debug)]
pub struct StateContext<'a> {
    plugin: &'a Plugin<'a>,
    state_context: NonNull<clap_plugin_state_context>,
}

impl<'a> Extension<&'a Plugin<'a>> for StateContext<'a> {
    const EXTENSION_ID: &'static CStr = CLAP_EXT_STATE_CONTEXT;

    type Struct = clap_plugin_state_context;

    fn new(plugin: &'a Plugin<'a>, extension_struct: NonNull<Self::Struct>) -> Self {
        Self {
            plugin,
            state_context: extension_struct,
        }
    }
}

impl StateContext<'_> {
    /// Save state for a specific context type. Returns an error if the plugin returned `false`.
    pub fn save(&self, context_type: clap_plugin_state_context_type) -> Result<Vec<u8>> {
        let stream = OutputStream::new();

        let state_context = self.state_context.as_ptr();
        let plugin = self.plugin.as_ptr();
        if unsafe_clap_call! { state_context=>save(plugin, stream.vtable(), context_type) } {
            Ok(stream.into_vec())
        } else {
            anyhow::bail!(
                "'clap_plugin_state_context::save()' returned false for context type \
                 {context_type}."
            );
        }
    }

    /// Load state for a specific context type. Returns an error if the plugin returned `false`.
    pub fn load(
        &self,
        state: &[u8],
        context_type: clap_plugin_state_context_type,
    ) -> Result<()> {
        let stream = InputStream::new(state);

        let state_context = self.state_context.as_ptr();
        let plugin = self.plugin.as_ptr();
        if unsafe_clap_call! { state_context=>load(plugin, stream.vtable(), context_type) } {
            Ok(())
        } else {
            anyhow::bail!(
                "'clap_plugin_state_context::load()' returned false for context type \
                 {context_type}."
            );
        }
    }
}
