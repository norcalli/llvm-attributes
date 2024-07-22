pub use strum::IntoEnumIterator;
use strum::{EnumIter, EnumProperty};

#[derive(Copy, Clone, Debug)]
pub struct LLVMAttributeInfo {
    pub description: &'static str,
    pub value_description: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, EnumIter, EnumProperty)]
pub enum LLVMAttribute {
    #[strum(props(
        Description = "Suggests that the function should always be inlined.",
        Value = "None"
    ))]
    AlwaysInline,

    #[strum(props(
        Description = "Suggests that the function should be inlined if possible.",
        Value = "None"
    ))]
    InlineHint,

    #[strum(props(
        Description = "Indicates that the function does not return.",
        Value = "None"
    ))]
    NoReturn,

    #[strum(props(
        Description = "Indicates that the function does not unwind the stack.",
        Value = "None"
    ))]
    NoUnwind,

    #[strum(props(
        Description = "Indicates that the function does not read or write memory.",
        Value = "None"
    ))]
    ReadNone,

    #[strum(props(
        Description = "Indicates that the function only reads memory but does not write.",
        Value = "None"
    ))]
    ReadOnly,

    #[strum(props(
        Description = "Indicates that the function only writes memory but does not read.",
        Value = "None"
    ))]
    WriteOnly,

    #[strum(props(
        Description = "Indicates that the function can be speculatively executed.",
        Value = "None"
    ))]
    Speculatable,

    #[strum(props(
        Description = "Suggests optimizing for minimum code size.",
        Value = "None"
    ))]
    MinSize,

    #[strum(props(Description = "Suggests optimizing for code size.", Value = "None"))]
    OptSize,

    #[strum(props(
        Description = "Suggests that the function should not be inlined.",
        Value = "None"
    ))]
    NoInline,

    #[strum(props(
        Description = "Indicates that the pointer argument is not captured.",
        Value = "None"
    ))]
    NoCapture,

    #[strum(props(
        Description = "Indicates that the pointer argument is not null.",
        Value = "None"
    ))]
    NonNull,

    #[strum(props(
        Description = "Indicates that the pointer argument is dereferenceable to the given size.",
        Value = "Integer (size in bytes)"
    ))]
    Dereferenceable,

    #[strum(props(
        Description = "Indicates that the pointer argument is either null or dereferenceable to the given size.",
        Value = "Integer (size in bytes)"
    ))]
    DereferenceableOrNull,

    #[strum(props(
        Description = "Indicates that the argument is a structure return pointer.",
        Value = "None"
    ))]
    SRet,

    #[strum(props(
        Description = "Specifies the alignment of the parameter or return value.",
        Value = "Integer (alignment in bytes)"
    ))]
    Align,

    #[strum(props(
        Description = "Indicates the allocation size for memory allocation functions.",
        Value = "Two integers (element size and number of elements)"
    ))]
    AllocSize,

    #[strum(props(
        Description = "Specifies the alignment of memory allocation.",
        Value = "Integer (alignment in bytes)"
    ))]
    Allocalign,

    #[strum(props(
        Description = "Indicates that the argument is also a return value.",
        Value = "None"
    ))]
    Returned,

    #[strum(props(
        Description = "Indicates that the integer argument should be zero-extended.",
        Value = "None"
    ))]
    ZeroExt,

    #[strum(props(
        Description = "Indicates that the integer argument should be sign-extended.",
        Value = "None"
    ))]
    SignExt,

    #[strum(props(
        Description = "Indicates that the function is unlikely to be executed.",
        Value = "None"
    ))]
    Cold,

    #[strum(props(
        Description = "Indicates that the function is likely to be executed.",
        Value = "None"
    ))]
    Hot,

    #[strum(props(
        Description = "Indicates that the function is not a built-in function.",
        Value = "None"
    ))]
    NoBuiltin,

    #[strum(props(
        Description = "Indicates that the function does not use a red zone.",
        Value = "None"
    ))]
    NoRedZone,

    #[strum(props(
        Description = "Enables AddressSanitizer for the function.",
        Value = "None"
    ))]
    SanitizeAddress,

    #[strum(props(
        Description = "Enables ThreadSanitizer for the function.",
        Value = "None"
    ))]
    SanitizeThread,

    #[strum(props(
        Description = "Enables MemorySanitizer for the function.",
        Value = "None"
    ))]
    SanitizeMemory,

    #[strum(props(
        Description = "Enables HardwareAddressSanitizer for the function.",
        Value = "None"
    ))]
    SanitizeHWAddress,

    #[strum(props(
        Description = "Enables strict floating-point semantics.",
        Value = "None"
    ))]
    StrictFP,

    #[strum(props(Description = "Enables stack protection.", Value = "None"))]
    StackProtector,

    #[strum(props(Description = "Requires stack protection.", Value = "None"))]
    StackProtectorReq,

    #[strum(props(Description = "Enables strong stack protection.", Value = "None"))]
    StackProtectorStrong,

    #[strum(props(
        Description = "Indicates that the function should have an unwind table.",
        Value = "None"
    ))]
    UWTable,

    #[strum(props(
        Description = "Indicates that the function may return more than once.",
        Value = "None"
    ))]
    ReturnsTwice,

    #[strum(props(
        Description = "Used in Swift to indicate the self parameter.",
        Value = "None"
    ))]
    SwiftSelf,

    #[strum(props(
        Description = "Used in Swift to indicate an error parameter.",
        Value = "None"
    ))]
    SwiftError,
}

impl LLVMAttribute {
    pub fn info(&self) -> LLVMAttributeInfo {
        LLVMAttributeInfo {
            description: self.get_str("Description").unwrap_or(""),
            value_description: self.get_str("Value").filter(|s| *s != "None"),
        }
    }
}
