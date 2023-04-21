#[macro_use]
extern crate lazy_static;

use neon::prelude::*;
use pyaco_core::Lang;
use pyaco_generate::{run as run_generate, Options as GenerateOptions};
use pyaco_validate::{run as run_validate, Options as ValidateOptions};
use tokio::runtime::Runtime;

lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

// macro_rules! get {
//     ($cx:ident, $options:ident, $name:expr, $type:ty) => {
//         $options
//             .get(&mut $cx, $name)?
//             .downcast_or_throw::<$type, _>(&mut $cx)?
//             .value(&mut $cx)
//     };

//     ($cx:ident, $options:ident, $name:expr) => {
//         get!($cx, $options, $name, JsString)
//     };
// }

fn generate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let options = cx.argument::<JsObject>(0)?;

    let input = options
        .get::<JsString, FunctionContext, _>(&mut cx, "input")?
        .value(&mut cx);

    let lang = options.get::<JsString, FunctionContext, _>(&mut cx, "lang");

    let lang = match lang {
        Err(_) => return cx.throw_error("Invalid lang"),
        Ok(lang) => {
            let lang = lang.value(&mut cx);

            match lang.parse::<Lang>() {
                Ok(lang) => lang,
                Err(err) => return cx.throw_error(err),
            }
        }
    };

    let output_directory = options
        .get::<JsString, FunctionContext, _>(&mut cx, "outputDirectory")?
        .value(&mut cx);

    let output_filename = options
        .get::<JsString, FunctionContext, _>(&mut cx, "outputFilename")?
        .value(&mut cx);

    let watch = options
        .get::<JsBoolean, FunctionContext, _>(&mut cx, "watch")?
        .value(&mut cx);

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

    let capture_regex = options
        .get::<JsString, FunctionContext, _>(&mut cx, "captureRegex")?
        .value(&mut cx);

    let css_input = options
        .get::<JsString, FunctionContext, _>(&mut cx, "cssInput")?
        .value(&mut cx);

    let input_glob = options
        .get::<JsString, FunctionContext, _>(&mut cx, "inputGlob")?
        .value(&mut cx);

    // The following will not panic, but the result is not reliable and might
    // change depending on the platform (32/64 bits).
    // Since we don't expect big numbers to be provided it should work fine though.
    let max_opened_files = options
        .get::<JsNumber, FunctionContext, _>(&mut cx, "maxOpenedFiles")?
        .value(&mut cx) as usize;

    let split_regex = options
        .get::<JsString, FunctionContext, _>(&mut cx, "splitRegex")?
        .value(&mut cx);

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
