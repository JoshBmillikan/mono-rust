
//todo debug-mono-symfile - needs glib.h
//todo profiler-events requires macro definitions, not sure if portable
//todo sgen-bridge
//todo verify.h - needs glib.h as well

pub mod jit {
    pub mod jit;
}
pub mod metadata {
    pub mod appdomain;
    pub mod assembly;
    pub mod attrdef;
    pub mod blob;
    pub mod class;
    pub mod debug_helpers;
    pub mod exception;
    pub mod image;
    pub mod loader;
    pub mod metadata;
    pub mod mono_config;
    pub mod mono_debug;
    pub mod mono_gc;
    pub mod object;
    pub mod object_forward;
    pub mod opcodes;
    pub mod profiler;
    pub mod reflection;
    pub mod row_indexes;
    pub mod threads;
    pub mod tokentype;
}

pub mod utils {
    pub mod mono_counters;
    pub mod mono_dl_fallback;
    pub mod mono_error;
    pub mod mono_forward;
    pub mod mono_jemalloc;
    pub mod mono_logger;
    pub mod mono_publib;
}

