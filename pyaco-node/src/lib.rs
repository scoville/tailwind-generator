#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// Limitations of napi
#![allow(clippy::needless_pass_by_value, clippy::inline_always)]

use napi::{
    bindgen_prelude::{Error, Result},
    CallContext, JsObject, JsUnknown,
};
use napi_derive::{js_function, module_exports};
use pyaco_generate::{run as run_generate, Options as GenerateOptions};
use pyaco_validate::{run as run_validate, Options as ValidateOptions};

#[js_function(1)]
fn generate(ctx: CallContext<'_>) -> Result<JsObject> {
    let options: GenerateOptions = ctx.env.from_js_value(ctx.get::<JsUnknown>(0).unwrap())?;

    ctx.env.execute_tokio_future(
        async move {
            match run_generate(options).await {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::from_reason("couldn't generate code")),
            }
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(1)]
fn validate(ctx: CallContext<'_>) -> Result<JsObject> {
    let options: ValidateOptions = ctx.env.from_js_value(ctx.get::<JsUnknown>(0).unwrap())?;

    ctx.env.execute_tokio_future(
        async move {
            if run_validate(options).await.is_err() {
                return Err(Error::from_reason("couldn't validate code"));
            };

            Ok(())
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    tracing_subscriber::fmt::init();

    exports.create_named_method("generate", generate)?;
    exports.create_named_method("validate", validate)?;

    Ok(())
}
