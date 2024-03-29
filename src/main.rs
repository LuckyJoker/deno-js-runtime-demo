use deno_core::{error::AnyError, op2, Extension, Op};
use std::{borrow::Cow, rc::Rc};

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
#[string]
async fn op_write_file(#[string] path: String, #[string] contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[op2(async)]
#[string]
async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
    let resp = reqwest::get(&url).await?.text().await?;
    Ok(resp)
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, std::env::current_dir()?.as_path())?;
    let runjs_extension = Extension {
        name: "runjs",
        ops: Cow::from(vec![
            op_read_file::DECL,
            op_write_file::DECL,
            op_remove_file::DECL,
            op_fetch::DECL,
        ]),
        ..Default::default()
    };

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    js_runtime
        .execute_script(
            "[runjs:runtime.js]",
            deno_core::FastString::from_static(include_str!("./runtime.js")),
        )
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js("./example.js")) {
        eprint!("error: {}\n", error);
    }
}
