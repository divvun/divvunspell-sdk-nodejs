use divvunspell::archive::{SpellerArchive, ZipSpellerArchive};
use divvunspell::speller::SpellerConfig;
use neon::prelude::*;
use std::{path::Path, sync::Arc};

pub struct SpellChecker {
    archive: Arc<ZipSpellerArchive>,
}

impl Finalize for SpellChecker {}

fn create_spellchecker(mut ctx: FunctionContext) -> JsResult<JsBox<SpellChecker>> {
    let zhfst_path = ctx.argument::<JsString>(0)?.value(&mut ctx);
    let archive = match ZipSpellerArchive::open(Path::new(&zhfst_path)) {
        Ok(v) => Arc::new(v),
        Err(e) => panic!("{:?}", e),
    };

    Ok(ctx.boxed(SpellChecker { archive }))
}

fn locale(mut ctx: FunctionContext) -> JsResult<JsValue> {
    let this = ctx.argument::<JsBox<SpellChecker>>(0)?;
    let archive = &this.archive;
    if let Some(meta) = archive.metadata() {
        Ok(ctx.string(&meta.info.locale).upcast())
    } else {
        Ok(ctx.null().upcast())
    }
}

fn locale_name(mut ctx: FunctionContext) -> JsResult<JsValue> {
    let this = ctx.argument::<JsBox<SpellChecker>>(0)?;
    let archive = &this.archive;
    if let Some(meta) = archive.metadata() {
        let locale = &meta.info.locale;
        let name = meta
            .info
            .title
            .iter()
            .find(|x| x.lang.as_ref().map(|l| l == locale).unwrap_or(false))
            .map(|x| x.value.to_string())
            .unwrap_or(meta.info.title[0].value.to_string());
        Ok(ctx.string(name).upcast())
    } else {
        Ok(ctx.null().upcast())
    }
}

fn is_correct(mut ctx: FunctionContext) -> JsResult<JsPromise> {
    let this = ctx.argument::<JsBox<SpellChecker>>(0)?;
    let word = ctx.argument::<JsString>(1)?.value(&mut ctx);
    let archive = this.archive.clone();

    let promise = ctx
        .task(move || archive.speller().is_correct(&word))
        .promise(|mut ctx, result| Ok(ctx.boolean(result)));
    Ok(promise)
}

fn suggest(mut func_ctx: FunctionContext) -> JsResult<JsPromise> {
    let this = func_ctx.argument::<JsBox<SpellChecker>>(0)?;
    let archive = this.archive.clone();
    let word = func_ctx.argument::<JsString>(1)?.value(&mut func_ctx);

    let promise = func_ctx
        .task(move || {
            let cfg = SpellerConfig::default();
            let result = archive
                .speller()
                .suggest_with_config(&word, &cfg)
                .into_iter()
                .map(|w| w.value.to_string())
                .collect::<Vec<String>>();
            Ok(result)
        })
        .promise(|mut ctx, result: Result<Vec<String>, std::io::Error>| {
            let suggs = result.unwrap();
            let output = JsArray::new(&mut ctx, suggs.len());
            for (i, obj) in suggs.into_iter().enumerate() {
                let s = obj;
                let v = ctx.string(&s);
                output.set(&mut ctx, i as u32, v).unwrap();
            }
            Ok(output)
        });
    Ok(promise)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut ctx: ModuleContext) -> NeonResult<()> {
    ctx.export_function("createSpellChecker", create_spellchecker)?;
    ctx.export_function("locale", locale)?;
    ctx.export_function("localeName", locale_name)?;
    ctx.export_function("isCorrect", is_correct)?;
    ctx.export_function("suggest", suggest)?;
    ctx.export_function("hello", hello)?;
    Ok(())
}
