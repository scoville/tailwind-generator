#[macro_use]
extern crate lazy_static;

use neon::prelude::*;
use pyaco_generate::{run as run_generate, Options as GenerateOptions};
use pyaco_validate::{run as run_validate, Options as ValidateOptions};
use tokio::runtime::Runtime;

lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

macro_rules! get {
    ($cx:ident, $options:ident, $name:expr, $type:ty) => {
        $options
            .get(&mut $cx, $name)?
            .downcast_or_throw::<$type, _>(&mut $cx)?
            .value(&mut $cx);
    };

    ($cx:ident, $options:ident, $name:expr) => {
        get!($cx, $options, $name, JsString);
    };
}

fn generate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let options = cx.argument::<JsObject>(0)?;

    let input = get!(cx, options, "input");

    let lang = get!(cx, options, "lang");

    let lang = match lang.parse() {
        Err(_) => return cx.throw_error("Invalid lang"),
        Ok(lang) => lang,
    };

    let output_directory = get!(cx, options, "outputDirectory");

    let output_filename = get!(cx, options, "outputFilename");

    let watch = get!(cx, options, "watch", JsBoolean);

    let options = GenerateOptions {
        input,
        lang,
        output_directory,
        output_filename,
        watch,
    };

    match run_generate(options) {
        Ok(_) => Ok(cx.undefined()),
        Err(_) => cx.throw_error("Couldn't generate code"),
    }
}

fn validate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let options = cx.argument::<JsObject>(0)?;

    let cb = cx.argument::<JsFunction>(1)?;

    let capture_regex = get!(cx, options, "captureRegex");

    let css_input = get!(cx, options, "cssInput");

    let input_glob = get!(cx, options, "inputGlob");

    // The following will not panic, but the result is not reliable and might
    // change depending on the platform (32/64 bits).
    // Since we don't expect big numbers to be provided it should work fine though.
    let max_opened_files = get!(cx, options, "maxOpenedFiles", JsNumber) as usize;

    let split_regex = get!(cx, options, "splitRegex");

    let options = ValidateOptions {
        capture_regex,
        css_input,
        input_glob,
        max_opened_files,
        split_regex,
    };

    let ret = cx.undefined();

    TOKIO_RUNTIME.block_on(async move {
        if run_validate(options).await.is_err() {
            return cx.throw_error("Couldn't validate code");
        };

        let this = cx.undefined();

        let args: Vec<Handle<JsUndefined>> = Vec::new();

        cb.call(&mut cx, this, args)?;

        Ok(())
    })?;

    Ok(ret)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    env_logger::init();

    cx.export_function("generate", generate)?;

    cx.export_function("validate", validate)?;

    Ok(())
}
