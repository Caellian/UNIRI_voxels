#[macro_export]
macro_rules! decl_id_type {
    ($name: ident) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(untagged)]
        pub enum $name {
            Dynamic(String),
            Static(&'static str),
        }

        impl $name {
            #[must_use]
            pub fn new(id: impl AsRef<str>) -> $name {
                $name::Dynamic(id.as_ref().to_string())
            }

            #[must_use]
            pub const fn new_static(id: &'static str) -> $name {
                $name::Static(id)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match self {
                    $name::Dynamic(it) => it.as_ref(),
                    $name::Static(it) => it,
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.as_ref() == other.as_ref()
            }
        }
        impl Eq for $name {}
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_ref().cmp(other.as_ref())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.write_str(self.as_ref())
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.as_ref().hash(state)
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }
    };
}

pub const fn weak_str_handle<T>(value: &str) -> bevy::asset::Handle<T>
where
    T: bevy::asset::Asset,
{
    let mut bytes = 0;

    let mut i = 0;
    loop {
        let c = value.as_bytes()[i];
        let mut j = 0u8;
        loop {
            bytes ^= (c as u128)
                .wrapping_shl(i as u32)
                .wrapping_add((j as u128 + 1).wrapping_mul(i as u128 + 1));
            j += 1;
            if j == 16 {
                break;
            }
        }
        i += 1;
        if value.len() <= i {
            break;
        }
    }

    bevy::asset::Handle::weak_from_u128(bytes)
}

pub(crate) struct If<const B: bool>;
pub trait True {}
impl True for If<true> {}
