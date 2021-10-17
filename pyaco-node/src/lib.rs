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

macro_rules! get_attribute {
    ($cx:ident[$index:literal][$name:literal] as $type:ty) => {
        $cx.argument::<JsObject>($index)?
            .get(&mut $cx, $name)?
            .downcast_or_throw::<$type, _>(&mut $cx)?
            .value(&mut $cx);
    };

    ($cx:ident[$index:literal][$name:literal]) => {
        get_attribute!($cx[$index][$name] as JsString);
    };

    ($cx:ident[$name:literal]) => {
        get_attribute!($cx[0][$name] as JsString);
    };

    ($cx:ident[$name:literal] as $type:ty) => {
        get_attribute!($cx[0][$name] as $type);
    };
}

fn generate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let input = get_attribute!(cx["input"]);

    let lang = get_attribute!(cx["lang"]);

    let lang = match lang.parse() {
        Err(_) => return cx.throw_error("Invalid lang"),
        Ok(lang) => lang,
    };

    let output_directory = get_attribute!(cx["outputDirectory"]);

    let output_filename = get_attribute!(cx["outputFilename"]);

    let watch = get_attribute!(cx["watch"] as JsBoolean);

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
    let cb = cx.argument::<JsFunction>(1)?;

    let capture_regex = get_attribute!(cx["captureRegex"]);

    let css_input = get_attribute!(cx["cssInput"]);

    let input_glob = get_attribute!(cx["inputGlob"]);

    // The following will not panic, but the result is not reliable and might
    // change depending on the platform (32/64 bits).
    // Since we don't expect big numbers to be provided it should work fine though.
    let max_opened_files = get_attribute!(cx["maxOpenedFiles"] as JsNumber) as usize;

    let split_regex = get_attribute!(cx["splitRegex"]);

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
