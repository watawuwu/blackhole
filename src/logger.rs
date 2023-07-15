use anyhow::Result;
use colored_json::{ColorMode, ColoredFormatter, CompactFormatter};
use fern::FormatCallback;
use log::LevelFilter;
use serde_json::{json, Value};
use std::fmt::Arguments;

pub fn init(no_color: bool, level_filter: LevelFilter) -> Result<()> {
    let out = std::io::stdout();
    let mut dispatch = fern::Dispatch::new()
        .level(level_filter)
        .level_for("actix_server", log::LevelFilter::Warn)
        .chain(out);

    if !no_color {
        let callback = |out: FormatCallback, message: &Arguments, _record: &log::Record| {
            let message = message.to_string();
            let value: Value = serde_json::from_str(message.as_str())
                .unwrap_or_else(|_| json!(format_args!(r#"{{"message": {}}}"#, message)));
            let formatter = ColoredFormatter::new(CompactFormatter {});
            let log = formatter
                .to_colored_json(&value, ColorMode::On)
                .unwrap_or_else(|_| value.to_string());
            out.finish(format_args!("{}", log))
        };

        dispatch = dispatch.format(callback);
    }

    Ok(dispatch.apply()?)
}
