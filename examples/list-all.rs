use llvm_attributes::{IntoEnumIterator, LLVMAttribute};

fn main() {
    for attr in LLVMAttribute::iter() {
        let info = attr.info();
        println!(
            "{attr:?} ({attr}) {}\n",
            format!("{info:#?}").replace("LLVMAttributeInfo ", "")
        );
    }
}
