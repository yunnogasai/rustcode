use std::mem::{size_of, size_of_val};

trait Trait {}

struct Struct;

impl Trait for Struct {}
fn main() {
    let yuno = 728_u32;
    let gasai = &yuno;
    println!("{gasai:p}"); //0x1234567890

    let str: &str = "rust";
    println!("{str:p}"); //Pointer { addr: 0x123456789012, metadata: 4 }

    let struct_ = Struct;
    let trait_obj: &dyn Trait = &struct_; //Pointer { addr: 0x1234567890, metadata: DynMetadata(0x123456789012) }
    println!("{trait_obj:p}");

    assert_eq!(size_of_val(trait_obj), 0);
    assert_eq!(size_of_val(str), str.len());
    assert_eq!(size_of::<&str>(), 16);
    assert_eq!(size_of::<&dyn Trait>(), 16);
    assert_eq!(size_of::<&str>(), size_of::<&dyn Trait>());
}
