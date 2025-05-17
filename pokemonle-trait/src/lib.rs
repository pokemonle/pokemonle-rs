// pokemonle-trait/src/lib.rs
// pub use pokemonle_derive::StructName; // 重新导出宏

pub trait StructName {
    fn struct_name() -> &'static str;

    fn tags() -> &'static [&'static str] {
        &[]
    }
}
