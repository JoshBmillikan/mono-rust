/* automatically generated by rust-bindgen 0.56.0 */

pub const MonoTypeEnum_MONO_TYPE_END: MonoTypeEnum = 0;
pub const MonoTypeEnum_MONO_TYPE_VOID: MonoTypeEnum = 1;
pub const MonoTypeEnum_MONO_TYPE_BOOLEAN: MonoTypeEnum = 2;
pub const MonoTypeEnum_MONO_TYPE_CHAR: MonoTypeEnum = 3;
pub const MonoTypeEnum_MONO_TYPE_I1: MonoTypeEnum = 4;
pub const MonoTypeEnum_MONO_TYPE_U1: MonoTypeEnum = 5;
pub const MonoTypeEnum_MONO_TYPE_I2: MonoTypeEnum = 6;
pub const MonoTypeEnum_MONO_TYPE_U2: MonoTypeEnum = 7;
pub const MonoTypeEnum_MONO_TYPE_I4: MonoTypeEnum = 8;
pub const MonoTypeEnum_MONO_TYPE_U4: MonoTypeEnum = 9;
pub const MonoTypeEnum_MONO_TYPE_I8: MonoTypeEnum = 10;
pub const MonoTypeEnum_MONO_TYPE_U8: MonoTypeEnum = 11;
pub const MonoTypeEnum_MONO_TYPE_R4: MonoTypeEnum = 12;
pub const MonoTypeEnum_MONO_TYPE_R8: MonoTypeEnum = 13;
pub const MonoTypeEnum_MONO_TYPE_STRING: MonoTypeEnum = 14;
pub const MonoTypeEnum_MONO_TYPE_PTR: MonoTypeEnum = 15;
pub const MonoTypeEnum_MONO_TYPE_BYREF: MonoTypeEnum = 16;
pub const MonoTypeEnum_MONO_TYPE_VALUETYPE: MonoTypeEnum = 17;
pub const MonoTypeEnum_MONO_TYPE_CLASS: MonoTypeEnum = 18;
pub const MonoTypeEnum_MONO_TYPE_VAR: MonoTypeEnum = 19;
pub const MonoTypeEnum_MONO_TYPE_ARRAY: MonoTypeEnum = 20;
pub const MonoTypeEnum_MONO_TYPE_GENERICINST: MonoTypeEnum = 21;
pub const MonoTypeEnum_MONO_TYPE_TYPEDBYREF: MonoTypeEnum = 22;
pub const MonoTypeEnum_MONO_TYPE_I: MonoTypeEnum = 24;
pub const MonoTypeEnum_MONO_TYPE_U: MonoTypeEnum = 25;
pub const MonoTypeEnum_MONO_TYPE_FNPTR: MonoTypeEnum = 27;
pub const MonoTypeEnum_MONO_TYPE_OBJECT: MonoTypeEnum = 28;
pub const MonoTypeEnum_MONO_TYPE_SZARRAY: MonoTypeEnum = 29;
pub const MonoTypeEnum_MONO_TYPE_MVAR: MonoTypeEnum = 30;
pub const MonoTypeEnum_MONO_TYPE_CMOD_REQD: MonoTypeEnum = 31;
pub const MonoTypeEnum_MONO_TYPE_CMOD_OPT: MonoTypeEnum = 32;
pub const MonoTypeEnum_MONO_TYPE_INTERNAL: MonoTypeEnum = 33;
pub const MonoTypeEnum_MONO_TYPE_MODIFIER: MonoTypeEnum = 64;
pub const MonoTypeEnum_MONO_TYPE_SENTINEL: MonoTypeEnum = 65;
pub const MonoTypeEnum_MONO_TYPE_PINNED: MonoTypeEnum = 69;
pub const MonoTypeEnum_MONO_TYPE_ENUM: MonoTypeEnum = 85;
pub type MonoTypeEnum = ::std::os::raw::c_int;
pub const MonoMetaTableEnum_MONO_TABLE_MODULE: MonoMetaTableEnum = 0;
pub const MonoMetaTableEnum_MONO_TABLE_TYPEREF: MonoMetaTableEnum = 1;
pub const MonoMetaTableEnum_MONO_TABLE_TYPEDEF: MonoMetaTableEnum = 2;
pub const MonoMetaTableEnum_MONO_TABLE_FIELD_POINTER: MonoMetaTableEnum = 3;
pub const MonoMetaTableEnum_MONO_TABLE_FIELD: MonoMetaTableEnum = 4;
pub const MonoMetaTableEnum_MONO_TABLE_METHOD_POINTER: MonoMetaTableEnum = 5;
pub const MonoMetaTableEnum_MONO_TABLE_METHOD: MonoMetaTableEnum = 6;
pub const MonoMetaTableEnum_MONO_TABLE_PARAM_POINTER: MonoMetaTableEnum = 7;
pub const MonoMetaTableEnum_MONO_TABLE_PARAM: MonoMetaTableEnum = 8;
pub const MonoMetaTableEnum_MONO_TABLE_INTERFACEIMPL: MonoMetaTableEnum = 9;
pub const MonoMetaTableEnum_MONO_TABLE_MEMBERREF: MonoMetaTableEnum = 10;
pub const MonoMetaTableEnum_MONO_TABLE_CONSTANT: MonoMetaTableEnum = 11;
pub const MonoMetaTableEnum_MONO_TABLE_CUSTOMATTRIBUTE: MonoMetaTableEnum = 12;
pub const MonoMetaTableEnum_MONO_TABLE_FIELDMARSHAL: MonoMetaTableEnum = 13;
pub const MonoMetaTableEnum_MONO_TABLE_DECLSECURITY: MonoMetaTableEnum = 14;
pub const MonoMetaTableEnum_MONO_TABLE_CLASSLAYOUT: MonoMetaTableEnum = 15;
pub const MonoMetaTableEnum_MONO_TABLE_FIELDLAYOUT: MonoMetaTableEnum = 16;
pub const MonoMetaTableEnum_MONO_TABLE_STANDALONESIG: MonoMetaTableEnum = 17;
pub const MonoMetaTableEnum_MONO_TABLE_EVENTMAP: MonoMetaTableEnum = 18;
pub const MonoMetaTableEnum_MONO_TABLE_EVENT_POINTER: MonoMetaTableEnum = 19;
pub const MonoMetaTableEnum_MONO_TABLE_EVENT: MonoMetaTableEnum = 20;
pub const MonoMetaTableEnum_MONO_TABLE_PROPERTYMAP: MonoMetaTableEnum = 21;
pub const MonoMetaTableEnum_MONO_TABLE_PROPERTY_POINTER: MonoMetaTableEnum = 22;
pub const MonoMetaTableEnum_MONO_TABLE_PROPERTY: MonoMetaTableEnum = 23;
pub const MonoMetaTableEnum_MONO_TABLE_METHODSEMANTICS: MonoMetaTableEnum = 24;
pub const MonoMetaTableEnum_MONO_TABLE_METHODIMPL: MonoMetaTableEnum = 25;
pub const MonoMetaTableEnum_MONO_TABLE_MODULEREF: MonoMetaTableEnum = 26;
pub const MonoMetaTableEnum_MONO_TABLE_TYPESPEC: MonoMetaTableEnum = 27;
pub const MonoMetaTableEnum_MONO_TABLE_IMPLMAP: MonoMetaTableEnum = 28;
pub const MonoMetaTableEnum_MONO_TABLE_FIELDRVA: MonoMetaTableEnum = 29;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED6: MonoMetaTableEnum = 30;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED7: MonoMetaTableEnum = 31;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLY: MonoMetaTableEnum = 32;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYPROCESSOR: MonoMetaTableEnum = 33;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYOS: MonoMetaTableEnum = 34;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREF: MonoMetaTableEnum = 35;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFPROCESSOR: MonoMetaTableEnum = 36;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFOS: MonoMetaTableEnum = 37;
pub const MonoMetaTableEnum_MONO_TABLE_FILE: MonoMetaTableEnum = 38;
pub const MonoMetaTableEnum_MONO_TABLE_EXPORTEDTYPE: MonoMetaTableEnum = 39;
pub const MonoMetaTableEnum_MONO_TABLE_MANIFESTRESOURCE: MonoMetaTableEnum = 40;
pub const MonoMetaTableEnum_MONO_TABLE_NESTEDCLASS: MonoMetaTableEnum = 41;
pub const MonoMetaTableEnum_MONO_TABLE_GENERICPARAM: MonoMetaTableEnum = 42;
pub const MonoMetaTableEnum_MONO_TABLE_METHODSPEC: MonoMetaTableEnum = 43;
pub const MonoMetaTableEnum_MONO_TABLE_GENERICPARAMCONSTRAINT: MonoMetaTableEnum = 44;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED8: MonoMetaTableEnum = 45;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED9: MonoMetaTableEnum = 46;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED10: MonoMetaTableEnum = 47;
pub const MonoMetaTableEnum_MONO_TABLE_DOCUMENT: MonoMetaTableEnum = 48;
pub const MonoMetaTableEnum_MONO_TABLE_METHODBODY: MonoMetaTableEnum = 49;
pub const MonoMetaTableEnum_MONO_TABLE_LOCALSCOPE: MonoMetaTableEnum = 50;
pub const MonoMetaTableEnum_MONO_TABLE_LOCALVARIABLE: MonoMetaTableEnum = 51;
pub const MonoMetaTableEnum_MONO_TABLE_LOCALCONSTANT: MonoMetaTableEnum = 52;
pub const MonoMetaTableEnum_MONO_TABLE_IMPORTSCOPE: MonoMetaTableEnum = 53;
pub const MonoMetaTableEnum_MONO_TABLE_STATEMACHINEMETHOD: MonoMetaTableEnum = 54;
pub const MonoMetaTableEnum_MONO_TABLE_CUSTOMDEBUGINFORMATION: MonoMetaTableEnum = 55;
pub type MonoMetaTableEnum = ::std::os::raw::c_int;
