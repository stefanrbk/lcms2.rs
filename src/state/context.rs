use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::{plugins::Plugin, types::signatures, LCMS_VERSION};

use super::chunks::{
    adaption_state::AdaptionStateChunk,
    alarm_codes::AlarmCodesChunk,
    error_handler::{ErrorCode, LogErrorChunk},
    formatters::FormattersPluginChunk,
    intents::IntentsPluginChunk,
    interpolation::InterpolationPluginChunk,
    optimizations::OptimizationPluginChunk,
    parametric_curves::CurvesPluginChunk,
    tag::TagPluginChunk,
    tag_type::TagTypePluginChunk,
    transform::TransformPluginChunk,
};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) user_data: Option<Box<[u8]>>,
    pub(crate) error_handler: Box<LogErrorChunk>,
    pub(crate) alarm_codes: Box<AlarmCodesChunk>,
    pub(crate) adaption_state: Box<AdaptionStateChunk>,
    pub(crate) interpolation_plugin: Box<InterpolationPluginChunk>,
    pub(crate) curves_plugin: Box<CurvesPluginChunk>,
    pub(crate) formatters_plugin: Box<FormattersPluginChunk>,
    pub(crate) tag_types_plugin: Box<TagTypePluginChunk>,
    pub(crate) tags_plugin: Box<TagPluginChunk>,
    pub(crate) intents_plugin: Box<IntentsPluginChunk>,
    pub(crate) mpe_types_plugin: Box<TagTypePluginChunk>,
    pub(crate) optimization_plugin: Box<OptimizationPluginChunk>,
    pub(crate) transform_plugin: Box<TransformPluginChunk>,
}

pub static GLOBAL_CONTEXT: Lazy<Mutex<Context>> = Lazy::new(|| Mutex::new(Context::new(None)));

impl Context {
    pub fn new(data: Option<Box<[u8]>>) -> Self {
        let result = Context {
            user_data: data,
            error_handler: Default::default(),
            alarm_codes: Default::default(),
            adaption_state: Default::default(),
            interpolation_plugin: Default::default(),
            curves_plugin: Default::default(),
            formatters_plugin: Default::default(),
            tag_types_plugin: Default::default(),
            tags_plugin: Default::default(),
            intents_plugin: Default::default(),
            mpe_types_plugin: Default::default(),
            optimization_plugin: Default::default(),
            transform_plugin: Default::default(),
        };

        result
    }

    pub fn init_plugin(&mut self, plugin: &Plugin) -> Result<()> {
        let signal_error = |ctx: &mut Self, code: ErrorCode, text: String| -> Result<()> {
            ctx.signal_error(code, text.clone());
            Err(text)
        };

        let mut plugin = plugin;
        while plugin.next.is_some() {
            if plugin.magic != signatures::plugin_type::MAGIC {
                return signal_error(
                    self,
                    ErrorCode::UnknownExtension,
                    "Unrecognized plugin".to_string(),
                );
            }
            if plugin.expected_version > LCMS_VERSION {
                return signal_error(
                    self,
                    ErrorCode::UnknownExtension,
                    format!(
                        "plugin needs Little CMS {}, current version is {}",
                        plugin.expected_version, LCMS_VERSION
                    ),
                );
            }

            match plugin.r#type {
                signatures::plugin_type::INTERPOLATION => (),
                signatures::plugin_type::TAG_TYPE => (),
                signatures::plugin_type::TAG => (),
                signatures::plugin_type::FORMATTERS => (),
                signatures::plugin_type::RENDERING_INTENT => (),
                signatures::plugin_type::PARAMETRIC_CURVE => (),
                signatures::plugin_type::MULTI_PROCESS_ELEMENT => (),
                signatures::plugin_type::OPTIMIZATION => (),
                signatures::plugin_type::TRANSFORM => (),
                _ => {
                    return signal_error(
                        self,
                        ErrorCode::UnknownExtension,
                        format!("Unrecognized plugin type '{:?}'", plugin.r#type),
                    )
                }
            };

            plugin = plugin.next.as_ref().unwrap();
        }

        Ok(())
    }

    pub fn get_user_data(&self) -> Option<&Box<[u8]>> {
        self.user_data.as_ref()
    }

    pub fn signal_error(&mut self, code: ErrorCode, text: impl Into<String>) {
        let eh = self.error_handler.handler;
        eh(self, code, text.into())
    }
}
