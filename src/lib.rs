use std::fmt::Display;

pub use strum::IntoEnumIterator;
use strum::{EnumIter, EnumProperty};

#[derive(Copy, Clone, Debug)]
pub struct LLVMAttributeInfo {
    pub kind_name: &'static str,
    pub description: &'static str,
    pub value_description: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, EnumIter, EnumProperty)]
pub enum LLVMAttribute {
    /// Suggests that the function should always be inlined.
    #[strum(props(
        Description = "Suggests that the function should always be inlined.",
        Value = "None",
        KindName = "alwaysinline"
    ))]
    AlwaysInline,

    /// Suggests that the function should be inlined if possible.
    #[strum(props(
        Description = "Suggests that the function should be inlined if possible.",
        Value = "None",
        KindName = "inlinehint"
    ))]
    InlineHint,

    /// Indicates that the function does not return.
    #[strum(props(
        Description = "Indicates that the function does not return.",
        Value = "None",
        KindName = "noreturn"
    ))]
    NoReturn,

    /// Indicates that the function does not unwind the stack.
    #[strum(props(
        Description = "Indicates that the function does not unwind the stack.",
        Value = "None",
        KindName = "nounwind"
    ))]
    NoUnwind,

    /// Indicates that the function does not read or write memory.
    #[strum(props(
        Description = "Indicates that the function does not read or write memory.",
        Value = "None",
        KindName = "readnone"
    ))]
    ReadNone,

    /// Indicates that the function only reads memory but does not write.
    #[strum(props(
        Description = "Indicates that the function only reads memory but does not write.",
        Value = "None",
        KindName = "readonly"
    ))]
    ReadOnly,

    /// Indicates that the function only writes memory but does not read.
    #[strum(props(
        Description = "Indicates that the function only writes memory but does not read.",
        Value = "None",
        KindName = "writeonly"
    ))]
    WriteOnly,

    /// Indicates that the function can be speculatively executed.
    #[strum(props(
        Description = "Indicates that the function can be speculatively executed.",
        Value = "None",
        KindName = "speculatable"
    ))]
    Speculatable,

    /// Suggests optimizing for minimum code size.
    #[strum(props(
        Description = "Suggests optimizing for minimum code size.",
        Value = "None",
        KindName = "minsize"
    ))]
    MinSize,

    /// Suggests optimizing for code size.
    #[strum(props(
        Description = "Suggests optimizing for code size.",
        Value = "None",
        KindName = "optsize"
    ))]
    OptSize,

    /// Suggests that the function should not be inlined.
    #[strum(props(
        Description = "Suggests that the function should not be inlined.",
        Value = "None",
        KindName = "noinline"
    ))]
    NoInline,

    /// Indicates that the pointer argument is not captured.
    #[strum(props(
        Description = "Indicates that the pointer argument is not captured.",
        Value = "None",
        KindName = "nocapture"
    ))]
    NoCapture,

    /// Indicates that the pointer argument is not null.
    #[strum(props(
        Description = "Indicates that the pointer argument is not null.",
        Value = "None",
        KindName = "nonnull"
    ))]
    NonNull,

    /// Indicates that the pointer argument is dereferenceable to the given size.
    #[strum(props(
        Description = "Indicates that the pointer argument is dereferenceable to the given size.",
        Value = "Integer (size in bytes)",
        KindName = "dereferenceable"
    ))]
    Dereferenceable,

    /// Indicates that the pointer argument is either null or dereferenceable to the given size.
    #[strum(props(
        Description = "Indicates that the pointer argument is either null or dereferenceable to the given size.",
        Value = "Integer (size in bytes)",
        KindName = "dereferenceable_or_null"
    ))]
    DereferenceableOrNull,

    /// Indicates that the argument is a structure return pointer.
    #[strum(props(
        Description = "Indicates that the argument is a structure return pointer.",
        Value = "None",
        KindName = "sret"
    ))]
    SRet,

    /// Specifies the alignment of the parameter or return value.
    #[strum(props(
        Description = "Specifies the alignment of the parameter or return value.",
        Value = "Integer (alignment in bytes)",
        KindName = "align"
    ))]
    Align,

    /// Indicates the allocation size for memory allocation functions.
    #[strum(props(
        Description = "Indicates the allocation size for memory allocation functions.",
        Value = "Two integers (element size and number of elements)",
        KindName = "allocsize"
    ))]
    AllocSize,

    /// Specifies the alignment of memory allocation.
    #[strum(props(
        Description = "Specifies the alignment of memory allocation.",
        Value = "Integer (alignment in bytes)",
        KindName = "allocalign"
    ))]
    Allocalign,

    /// Indicates that the argument is also a return value.
    #[strum(props(
        Description = "Indicates that the argument is also a return value.",
        Value = "None",
        KindName = "returned"
    ))]
    Returned,

    /// Indicates that the integer argument should be zero-extended.
    #[strum(props(
        Description = "Indicates that the integer argument should be zero-extended.",
        Value = "None",
        KindName = "zeroext"
    ))]
    ZeroExt,

    /// Indicates that the integer argument should be sign-extended.
    #[strum(props(
        Description = "Indicates that the integer argument should be sign-extended.",
        Value = "None",
        KindName = "signext"
    ))]
    SignExt,

    /// Indicates that the function is unlikely to be executed.
    #[strum(props(
        Description = "Indicates that the function is unlikely to be executed.",
        Value = "None",
        KindName = "cold"
    ))]
    Cold,

    /// Indicates that the function is likely to be executed.
    #[strum(props(
        Description = "Indicates that the function is likely to be executed.",
        Value = "None",
        KindName = "hot"
    ))]
    Hot,

    /// Indicates that the function is not a built-in function.
    #[strum(props(
        Description = "Indicates that the function is not a built-in function.",
        Value = "None",
        KindName = "nobuiltin"
    ))]
    NoBuiltin,

    /// Indicates that the function does not use a red zone.
    #[strum(props(
        Description = "Indicates that the function does not use a red zone.",
        Value = "None",
        KindName = "noredzone"
    ))]
    NoRedZone,

    /// Enables AddressSanitizer for the function.
    #[strum(props(
        Description = "Enables AddressSanitizer for the function.",
        Value = "None",
        KindName = "sanitize_address"
    ))]
    SanitizeAddress,

    /// Enables ThreadSanitizer for the function.
    #[strum(props(
        Description = "Enables ThreadSanitizer for the function.",
        Value = "None",
        KindName = "sanitize_thread"
    ))]
    SanitizeThread,

    /// Enables MemorySanitizer for the function.
    #[strum(props(
        Description = "Enables MemorySanitizer for the function.",
        Value = "None",
        KindName = "sanitize_memory"
    ))]
    SanitizeMemory,

    /// Enables HardwareAddressSanitizer for the function.
    #[strum(props(
        Description = "Enables HardwareAddressSanitizer for the function.",
        Value = "None",
        KindName = "sanitize_hwaddress"
    ))]
    SanitizeHWAddress,

    /// Enables strict floating-point semantics.
    #[strum(props(
        Description = "Enables strict floating-point semantics.",
        Value = "None",
        KindName = "strictfp"
    ))]
    StrictFP,

    /// Enables stack protection.
    #[strum(props(
        Description = "Enables stack protection.",
        Value = "None",
        KindName = "stackprotector"
    ))]
    StackProtector,

    /// Requires stack protection.
    #[strum(props(
        Description = "Requires stack protection.",
        Value = "None",
        KindName = "stackprotectorreq"
    ))]
    StackProtectorReq,

    /// Enables strong stack protection.
    #[strum(props(
        Description = "Enables strong stack protection.",
        Value = "None",
        KindName = "stackprotectorstrong"
    ))]
    StackProtectorStrong,

    /// Indicates that the function should have an unwind table.
    #[strum(props(
        Description = "Indicates that the function should have an unwind table.",
        Value = "None",
        KindName = "uwtable"
    ))]
    UWTable,

    /// Indicates that the function may return more than once.
    #[strum(props(
        Description = "Indicates that the function may return more than once.",
        Value = "None",
        KindName = "returns_twice"
    ))]
    ReturnsTwice,

    /// Used in Swift to indicate the self parameter.
    #[strum(props(
        Description = "Used in Swift to indicate the self parameter.",
        Value = "None",
        KindName = "swiftself"
    ))]
    SwiftSelf,

    /// Used in Swift to indicate an error parameter.
    #[strum(props(
        Description = "Used in Swift to indicate an error parameter.",
        Value = "None",
        KindName = "swifterror"
    ))]
    SwiftError,
}

impl Display for LLVMAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.kind_name())
    }
}

impl LLVMAttribute {
    pub fn description(&self) -> &'static str {
        self.get_str("Description").unwrap()
    }

    pub fn value_description(&self) -> Option<&'static str> {
        self.get_str("Value").filter(|s| *s != "None")
    }

    pub fn value_description_str(&self) -> &'static str {
        self.get_str("Value").unwrap()
    }

    pub fn kind_name(&self) -> &'static str {
        self.get_str("KindName").unwrap()
    }
}

impl LLVMAttribute {
    pub fn info(&self) -> LLVMAttributeInfo {
        LLVMAttributeInfo {
            kind_name: self.kind_name(),
            description: self.description(),
            value_description: self.value_description(),
        }
    }
}
