#[macro_export]
macro_rules! define_extra_struct {
    ($name:ident { $($field:ident: $type:ty),* }) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema,aide::OperationIo)]
        pub struct $name<T> {
            #[serde(flatten)]
            pub item: T,
            $(
                pub $field: $type,
            )*
        }

        impl<T> pokemonle_trait::StructName for $name<T> where T: pokemonle_trait::StructName {
            fn struct_name() -> &'static str {
                T::struct_name()
            }

            fn tags() -> &'static [&'static str] {
                T::tags()
            }
        }
    };
}
