use std::{path::Path, sync::Arc};
use neon::prelude::*;
use divvunspell::speller::SpellerConfig;
use divvunspell::archive::{SpellerArchive, ZipSpellerArchive};

pub struct SpellChecker {
    archive: Arc<ZipSpellerArchive>
}

struct JsIsCorrectTask {
    archive: Arc<ZipSpellerArchive>,
    word: String
}

impl JsIsCorrectTask {
    pub fn new(archive: Arc<ZipSpellerArchive>, word: String) -> Self {
        Self { archive, word }
    }
}

impl Task for JsIsCorrectTask {
    type Output = bool;
    type Error = String;
    type JsEvent = JsBoolean;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        Ok(self.archive.speller().is_correct(&self.word))
    }

    fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        Ok(ctx.boolean(result.unwrap()))
    }
}

struct JsSuggestTask {
    archive: Arc<ZipSpellerArchive>,
    word: String
}

impl JsSuggestTask {
    pub fn new(archive: Arc<ZipSpellerArchive>, word: String) -> Self {
        Self { archive, word }
    }
}

impl Task for JsSuggestTask {
    type Output = Vec<String>;
    type Error = String;
    type JsEvent = JsArray;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let cfg = SpellerConfig::default();
        Ok(self.archive.speller()
            .suggest_with_config(&self.word, &cfg)
            .into_iter().map(|w| w.value.to_string())
            .collect())
    }

    fn complete(self, mut ctx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        let output = result.unwrap();
        let js_array = JsArray::new(&mut ctx, output.len() as u32);
        for (i, obj) in output.into_iter().enumerate() {
            let s = ctx.string(obj);
            js_array.set(&mut ctx, i as u32, s).unwrap();
        }
        Ok(js_array)
    }
}

declare_types! {
    pub class JsSpellChecker for SpellChecker {
        init(mut ctx) {
            let zhfst_path: Handle<JsString> = ctx.argument::<JsString>(0)?;
            let archive = match ZipSpellerArchive::open(&Path::new(&zhfst_path.value())) {
                Ok(v) => Arc::new(v),
                Err(e) => panic!("{:?}", e)
            };

            Ok(SpellChecker { archive })
        }

        method suggest(mut ctx) {
            let this = ctx.this();
            let arg0 = ctx.argument::<JsString>(0)?;
            let arg1 = ctx.argument::<JsFunction>(1)?;

            let word = arg0.value();
            let archive = {
                let guard = ctx.lock();
                let a = &this.borrow(&guard).archive;
                a.clone()
            };
            
            JsSuggestTask::new(archive, word).schedule(arg1);
            Ok(ctx.undefined().upcast())
        }

        method isCorrect(mut ctx) {
            let this = ctx.this();
            let arg0 = ctx.argument::<JsString>(0)?;
            let arg1 = ctx.argument::<JsFunction>(1)?;

            let word = arg0.value();
            let archive = {
                let guard = ctx.lock();
                let a = &this.borrow(&guard).archive;
                a.clone()
            };
            JsIsCorrectTask::new(archive, word).schedule(arg1);

            Ok(ctx.undefined().upcast())
        }

        method locale(mut ctx) {
            let this = ctx.this();
            
            let archive = {
                let guard = ctx.lock();
                let x = this.borrow(&guard);
                x.archive.clone()
            };

            if let Some(meta) = archive.metadata() {
                Ok(ctx.string(&meta.info.locale).upcast())
            } else {
                Ok(ctx.null().upcast())
            }
        }

        method localeName(mut ctx) {
            let this = ctx.this();
            
            let archive = {
                let guard = ctx.lock();
                let x = this.borrow(&guard);
                x.archive.clone()
            };

            if let Some(meta) = archive.metadata() {
                let locale = &meta.info.locale;
                let name = meta.info.title
                    .iter()
                    .find(|x| x.lang.as_ref().map(|l| l == locale).unwrap_or(false))
                    .map(|x| x.value.to_string())
                    .unwrap_or(meta.info.title[0].value.to_string());
                Ok(ctx.string(name).upcast())
            } else {
                Ok(ctx.null().upcast())
            }
        }
    }
}

register_module!(mut ctx, {
    ctx.export_class::<JsSpellChecker>("SpellChecker")
});
