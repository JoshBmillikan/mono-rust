/* automatically generated by rust-bindgen 0.56.0 */

pub const MONO_ASSEMBLY_HASH_NONE: ::std::os::raw::c_int = 0;
pub const MONO_ASSEMBLY_HASH_MD5: ::std::os::raw::c_int = 32771;
pub const MONO_ASSEMBLY_HASH_SHA1: ::std::os::raw::c_int = 32772;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub const MONO_ASSEMBLYREF_FULL_PUBLIC_KEY: ::std::os::raw::c_int = 1;
pub const MONO_ASSEMBLYREF_RETARGETABLE: ::std::os::raw::c_int = 256;
pub const MONO_ASSEMBLYREF_JIT_TRACKING: ::std::os::raw::c_int = 32768;
pub const MONO_ASSEMBLYREF_NO_JIT_OPT: ::std::os::raw::c_int = 16384;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub const MONO_EVENT_SPECIALNAME: ::std::os::raw::c_int = 512;
pub const MONO_EVENT_RTSPECIALNAME: ::std::os::raw::c_int = 1024;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub const MONO_FIELD_ATTR_FIELD_ACCESS_MASK: ::std::os::raw::c_int = 7;
pub const MONO_FIELD_ATTR_COMPILER_CONTROLLED: ::std::os::raw::c_int = 0;
pub const MONO_FIELD_ATTR_PRIVATE: ::std::os::raw::c_int = 1;
pub const MONO_FIELD_ATTR_FAM_AND_ASSEM: ::std::os::raw::c_int = 2;
pub const MONO_FIELD_ATTR_ASSEMBLY: ::std::os::raw::c_int = 3;
pub const MONO_FIELD_ATTR_FAMILY: ::std::os::raw::c_int = 4;
pub const MONO_FIELD_ATTR_FAM_OR_ASSEM: ::std::os::raw::c_int = 5;
pub const MONO_FIELD_ATTR_PUBLIC: ::std::os::raw::c_int = 6;
pub const MONO_FIELD_ATTR_STATIC: ::std::os::raw::c_int = 16;
pub const MONO_FIELD_ATTR_INIT_ONLY: ::std::os::raw::c_int = 32;
pub const MONO_FIELD_ATTR_LITERAL: ::std::os::raw::c_int = 64;
pub const MONO_FIELD_ATTR_NOT_SERIALIZED: ::std::os::raw::c_int = 128;
pub const MONO_FIELD_ATTR_SPECIAL_NAME: ::std::os::raw::c_int = 512;
pub const MONO_FIELD_ATTR_PINVOKE_IMPL: ::std::os::raw::c_int = 8192;
pub const MONO_FIELD_ATTR_RESERVED_MASK: ::std::os::raw::c_int = 38144;
pub const MONO_FIELD_ATTR_RT_SPECIAL_NAME: ::std::os::raw::c_int = 1024;
pub const MONO_FIELD_ATTR_HAS_MARSHAL: ::std::os::raw::c_int = 4096;
pub const MONO_FIELD_ATTR_HAS_DEFAULT: ::std::os::raw::c_int = 32768;
pub const MONO_FIELD_ATTR_HAS_RVA: ::std::os::raw::c_int = 256;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
pub const MONO_FILE_HAS_METADATA: ::std::os::raw::c_int = 0;
pub const MONO_FILE_HAS_NO_METADATA: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub const MONO_GEN_PARAM_VARIANCE_MASK: ::std::os::raw::c_int = 3;
pub const MONO_GEN_PARAM_NON_VARIANT: ::std::os::raw::c_int = 0;
pub const MONO_GEN_PARAM_VARIANT: ::std::os::raw::c_int = 1;
pub const MONO_GEN_PARAM_COVARIANT: ::std::os::raw::c_int = 2;
pub const MONO_GEN_PARAM_CONSTRAINT_MASK: ::std::os::raw::c_int = 28;
pub const MONO_GEN_PARAM_CONSTRAINT_CLASS: ::std::os::raw::c_int = 4;
pub const MONO_GEN_PARAM_CONSTRAINT_VTYPE: ::std::os::raw::c_int = 8;
pub const MONO_GEN_PARAM_CONSTRAINT_DCTOR: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
pub const MONO_PINVOKE_NO_MANGLE: ::std::os::raw::c_int = 1;
pub const MONO_PINVOKE_CHAR_SET_MASK: ::std::os::raw::c_int = 6;
pub const MONO_PINVOKE_CHAR_SET_NOT_SPEC: ::std::os::raw::c_int = 0;
pub const MONO_PINVOKE_CHAR_SET_ANSI: ::std::os::raw::c_int = 2;
pub const MONO_PINVOKE_CHAR_SET_UNICODE: ::std::os::raw::c_int = 4;
pub const MONO_PINVOKE_CHAR_SET_AUTO: ::std::os::raw::c_int = 6;
pub const MONO_PINVOKE_BEST_FIT_ENABLED: ::std::os::raw::c_int = 16;
pub const MONO_PINVOKE_BEST_FIT_DISABLED: ::std::os::raw::c_int = 32;
pub const MONO_PINVOKE_BEST_FIT_MASK: ::std::os::raw::c_int = 48;
pub const MONO_PINVOKE_SUPPORTS_LAST_ERROR: ::std::os::raw::c_int = 64;
pub const MONO_PINVOKE_CALL_CONV_MASK: ::std::os::raw::c_int = 1792;
pub const MONO_PINVOKE_CALL_CONV_WINAPI: ::std::os::raw::c_int = 256;
pub const MONO_PINVOKE_CALL_CONV_CDECL: ::std::os::raw::c_int = 512;
pub const MONO_PINVOKE_CALL_CONV_STDCALL: ::std::os::raw::c_int = 768;
pub const MONO_PINVOKE_CALL_CONV_THISCALL: ::std::os::raw::c_int = 1024;
pub const MONO_PINVOKE_CALL_CONV_FASTCALL: ::std::os::raw::c_int = 1280;
pub const MONO_PINVOKE_THROW_ON_UNMAPPABLE_ENABLED: ::std::os::raw::c_int = 4096;
pub const MONO_PINVOKE_THROW_ON_UNMAPPABLE_DISABLED: ::std::os::raw::c_int = 8192;
pub const MONO_PINVOKE_THROW_ON_UNMAPPABLE_MASK: ::std::os::raw::c_int = 12288;
pub const MONO_PINVOKE_CALL_CONV_GENERIC: ::std::os::raw::c_int = 16;
pub const MONO_PINVOKE_CALL_CONV_GENERICINST: ::std::os::raw::c_int = 10;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const MONO_MANIFEST_RESOURCE_VISIBILITY_MASK: ::std::os::raw::c_int = 7;
pub const MONO_MANIFEST_RESOURCE_PUBLIC: ::std::os::raw::c_int = 1;
pub const MONO_MANIFEST_RESOURCE_PRIVATE: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const MONO_METHOD_ATTR_ACCESS_MASK: ::std::os::raw::c_int = 7;
pub const MONO_METHOD_ATTR_COMPILER_CONTROLLED: ::std::os::raw::c_int = 0;
pub const MONO_METHOD_ATTR_PRIVATE: ::std::os::raw::c_int = 1;
pub const MONO_METHOD_ATTR_FAM_AND_ASSEM: ::std::os::raw::c_int = 2;
pub const MONO_METHOD_ATTR_ASSEM: ::std::os::raw::c_int = 3;
pub const MONO_METHOD_ATTR_FAMILY: ::std::os::raw::c_int = 4;
pub const MONO_METHOD_ATTR_FAM_OR_ASSEM: ::std::os::raw::c_int = 5;
pub const MONO_METHOD_ATTR_PUBLIC: ::std::os::raw::c_int = 6;
pub const MONO_METHOD_ATTR_STATIC: ::std::os::raw::c_int = 16;
pub const MONO_METHOD_ATTR_FINAL: ::std::os::raw::c_int = 32;
pub const MONO_METHOD_ATTR_VIRTUAL: ::std::os::raw::c_int = 64;
pub const MONO_METHOD_ATTR_HIDE_BY_SIG: ::std::os::raw::c_int = 128;
pub const MONO_METHOD_ATTR_VTABLE_LAYOUT_MASK: ::std::os::raw::c_int = 256;
pub const MONO_METHOD_ATTR_REUSE_SLOT: ::std::os::raw::c_int = 0;
pub const MONO_METHOD_ATTR_NEW_SLOT: ::std::os::raw::c_int = 256;
pub const MONO_METHOD_ATTR_STRICT: ::std::os::raw::c_int = 512;
pub const MONO_METHOD_ATTR_ABSTRACT: ::std::os::raw::c_int = 1024;
pub const MONO_METHOD_ATTR_SPECIAL_NAME: ::std::os::raw::c_int = 2048;
pub const MONO_METHOD_ATTR_PINVOKE_IMPL: ::std::os::raw::c_int = 8192;
pub const MONO_METHOD_ATTR_UNMANAGED_EXPORT: ::std::os::raw::c_int = 8;
pub const MONO_METHOD_ATTR_RESERVED_MASK: ::std::os::raw::c_int = 53248;
pub const MONO_METHOD_ATTR_RT_SPECIAL_NAME: ::std::os::raw::c_int = 4096;
pub const MONO_METHOD_ATTR_HAS_SECURITY: ::std::os::raw::c_int = 16384;
pub const MONO_METHOD_ATTR_REQUIRE_SEC_OBJECT: ::std::os::raw::c_int = 32768;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const MONO_METHOD_IMPL_ATTR_CODE_TYPE_MASK: ::std::os::raw::c_int = 3;
pub const MONO_METHOD_IMPL_ATTR_IL: ::std::os::raw::c_int = 0;
pub const MONO_METHOD_IMPL_ATTR_NATIVE: ::std::os::raw::c_int = 1;
pub const MONO_METHOD_IMPL_ATTR_OPTIL: ::std::os::raw::c_int = 2;
pub const MONO_METHOD_IMPL_ATTR_RUNTIME: ::std::os::raw::c_int = 3;
pub const MONO_METHOD_IMPL_ATTR_MANAGED_MASK: ::std::os::raw::c_int = 4;
pub const MONO_METHOD_IMPL_ATTR_UNMANAGED: ::std::os::raw::c_int = 4;
pub const MONO_METHOD_IMPL_ATTR_MANAGED: ::std::os::raw::c_int = 0;
pub const MONO_METHOD_IMPL_ATTR_FORWARD_REF: ::std::os::raw::c_int = 16;
pub const MONO_METHOD_IMPL_ATTR_PRESERVE_SIG: ::std::os::raw::c_int = 128;
pub const MONO_METHOD_IMPL_ATTR_INTERNAL_CALL: ::std::os::raw::c_int = 4096;
pub const MONO_METHOD_IMPL_ATTR_SYNCHRONIZED: ::std::os::raw::c_int = 32;
pub const MONO_METHOD_IMPL_ATTR_NOINLINING: ::std::os::raw::c_int = 8;
pub const MONO_METHOD_IMPL_ATTR_NOOPTIMIZATION: ::std::os::raw::c_int = 64;
pub const MONO_METHOD_IMPL_ATTR_MAX_METHOD_IMPL_VAL: ::std::os::raw::c_int = 65535;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const MONO_METHOD_SEMANTIC_SETTER: ::std::os::raw::c_int = 1;
pub const MONO_METHOD_SEMANTIC_GETTER: ::std::os::raw::c_int = 2;
pub const MONO_METHOD_SEMANTIC_OTHER: ::std::os::raw::c_int = 4;
pub const MONO_METHOD_SEMANTIC_ADD_ON: ::std::os::raw::c_int = 8;
pub const MONO_METHOD_SEMANTIC_REMOVE_ON: ::std::os::raw::c_int = 16;
pub const MONO_METHOD_SEMANTIC_FIRE: ::std::os::raw::c_int = 32;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const MONO_PARAM_ATTR_IN: ::std::os::raw::c_int = 1;
pub const MONO_PARAM_ATTR_OUT: ::std::os::raw::c_int = 2;
pub const MONO_PARAM_ATTR_OPTIONAL: ::std::os::raw::c_int = 16;
pub const MONO_PARAM_ATTR_RESERVED_MASK: ::std::os::raw::c_int = 61440;
pub const MONO_PARAM_ATTR_HAS_DEFAULT: ::std::os::raw::c_int = 4096;
pub const MONO_PARAM_ATTR_HAS_MARSHAL: ::std::os::raw::c_int = 8192;
pub const MONO_PARAM_ATTR_UNUSED: ::std::os::raw::c_int = 53216;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const MONO_PROPERTY_ATTR_SPECIAL_NAME: ::std::os::raw::c_int = 512;
pub const MONO_PROPERTY_ATTR_RESERVED_MASK: ::std::os::raw::c_int = 62464;
pub const MONO_PROPERTY_ATTR_RT_SPECIAL_NAME: ::std::os::raw::c_int = 1024;
pub const MONO_PROPERTY_ATTR_HAS_DEFAULT: ::std::os::raw::c_int = 4096;
pub const MONO_PROPERTY_ATTR_UNUSED: ::std::os::raw::c_int = 59903;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const MONO_TYPE_ATTR_VISIBILITY_MASK: ::std::os::raw::c_int = 7;
pub const MONO_TYPE_ATTR_NOT_PUBLIC: ::std::os::raw::c_int = 0;
pub const MONO_TYPE_ATTR_PUBLIC: ::std::os::raw::c_int = 1;
pub const MONO_TYPE_ATTR_NESTED_PUBLIC: ::std::os::raw::c_int = 2;
pub const MONO_TYPE_ATTR_NESTED_PRIVATE: ::std::os::raw::c_int = 3;
pub const MONO_TYPE_ATTR_NESTED_FAMILY: ::std::os::raw::c_int = 4;
pub const MONO_TYPE_ATTR_NESTED_ASSEMBLY: ::std::os::raw::c_int = 5;
pub const MONO_TYPE_ATTR_NESTED_FAM_AND_ASSEM: ::std::os::raw::c_int = 6;
pub const MONO_TYPE_ATTR_NESTED_FAM_OR_ASSEM: ::std::os::raw::c_int = 7;
pub const MONO_TYPE_ATTR_LAYOUT_MASK: ::std::os::raw::c_int = 24;
pub const MONO_TYPE_ATTR_AUTO_LAYOUT: ::std::os::raw::c_int = 0;
pub const MONO_TYPE_ATTR_SEQUENTIAL_LAYOUT: ::std::os::raw::c_int = 8;
pub const MONO_TYPE_ATTR_EXPLICIT_LAYOUT: ::std::os::raw::c_int = 16;
pub const MONO_TYPE_ATTR_CLASS_SEMANTIC_MASK: ::std::os::raw::c_int = 32;
pub const MONO_TYPE_ATTR_CLASS: ::std::os::raw::c_int = 0;
pub const MONO_TYPE_ATTR_INTERFACE: ::std::os::raw::c_int = 32;
pub const MONO_TYPE_ATTR_ABSTRACT: ::std::os::raw::c_int = 128;
pub const MONO_TYPE_ATTR_SEALED: ::std::os::raw::c_int = 256;
pub const MONO_TYPE_ATTR_SPECIAL_NAME: ::std::os::raw::c_int = 1024;
pub const MONO_TYPE_ATTR_IMPORT: ::std::os::raw::c_int = 4096;
pub const MONO_TYPE_ATTR_SERIALIZABLE: ::std::os::raw::c_int = 8192;
pub const MONO_TYPE_ATTR_STRING_FORMAT_MASK: ::std::os::raw::c_int = 196608;
pub const MONO_TYPE_ATTR_ANSI_CLASS: ::std::os::raw::c_int = 0;
pub const MONO_TYPE_ATTR_UNICODE_CLASS: ::std::os::raw::c_int = 65536;
pub const MONO_TYPE_ATTR_AUTO_CLASS: ::std::os::raw::c_int = 131072;
pub const MONO_TYPE_ATTR_CUSTOM_CLASS: ::std::os::raw::c_int = 196608;
pub const MONO_TYPE_ATTR_CUSTOM_MASK: ::std::os::raw::c_int = 12582912;
pub const MONO_TYPE_ATTR_BEFORE_FIELD_INIT: ::std::os::raw::c_int = 1048576;
pub const MONO_TYPE_ATTR_FORWARDER: ::std::os::raw::c_int = 2097152;
pub const MONO_TYPE_ATTR_RESERVED_MASK: ::std::os::raw::c_int = 264192;
pub const MONO_TYPE_ATTR_RT_SPECIAL_NAME: ::std::os::raw::c_int = 2048;
pub const MONO_TYPE_ATTR_HAS_SECURITY: ::std::os::raw::c_int = 262144;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;