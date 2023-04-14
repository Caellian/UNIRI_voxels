#[macro_export]
macro_rules! decl_id_type {
    ($name: ident) => {
        #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        #[repr(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(id: impl AsRef<str>) -> $name {
                $name(id.as_ref().to_string())
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }
    };
}
