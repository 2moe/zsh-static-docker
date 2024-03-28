use lang_id::LangID;
use std::sync::OnceLock;
use time::OffsetDateTime;
fn sys_lang() -> &'static LangID {
    static LANG: OnceLock<LangID> = OnceLock::new();
    LANG.get_or_init(lang_id::sys_lang::current)
}
pub(crate) fn os_region() -> &'static str {
    match sys_lang().region {
        Some(ref x) => x.as_str(),
        _ => "US",
    }
}

pub(crate) fn today() -> &'static str {
    static D: OnceLock<String> = OnceLock::new();
    D.get_or_init(|| now_time().date().to_string())
}

pub(crate) fn now_time() -> &'static OffsetDateTime {
    static D: OnceLock<OffsetDateTime> = OnceLock::new();
    D.get_or_init(time::OffsetDateTime::now_utc)
}
