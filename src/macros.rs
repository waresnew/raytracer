/// where each enum variant will have the same name as the struct it holds
macro_rules! gen_struct_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($variant($variant), )*
        }

        $(impl From<$variant> for $name {
            fn from(s:$variant)->Self {
                Self::$variant(s)
            }
        })*
    };
}
pub(crate) use gen_struct_enum;
