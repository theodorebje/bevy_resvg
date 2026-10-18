macro_rules! enum_def {
    // Base case: nothing left to process
    () => {};

    // Match an enum, expand it, then process the rest
    (
        $(#[$struct_meta:meta])*
        $vis:vis enum $name:ident -> $foreign:ident {
            $(
                $(#[$field_meta:meta])*
                $variant_name:ident
            ),* $(,)?
        }
        $($rest:tt)*
    ) => {
        pastey::paste! {
            #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
            #[serde(remote = "" $foreign "")]
            $(#[$struct_meta])*
            $vis enum $name {
                $(
                    $(#[$field_meta])*
                    $variant_name,
                )*
            }
        }

        impl From<$foreign> for $name {
            fn from(value: $foreign) -> Self {
                match value {
                    $(<$foreign>::$variant_name => Self::$variant_name,)*
                }
            }
        }

        impl From<$name> for $foreign {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant_name => <$foreign>::$variant_name,)*
                }
            }
        }


        impl Default for $name {
            fn default() -> Self {
                <$foreign>::default().into()
            }
        }

        // Recursively process the remaining tokens
        enum_def!($($rest)*);
    };
}

macro_rules! options_def {
    // Match a struct, expand it
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident -> $foreign:ty {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
        $($rest:tt)*
    ) => {
        #[derive(Debug, Serialize, Deserialize, Clone)]
        $(#[$struct_meta])*
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_vis $field_name : $field_ty,
            )*
        }

        impl From<$foreign> for $name {
            fn from(value: $foreign) -> Self {
                Self {
                    $($field_name: value.$field_name,)*
                }
            }
        }

        impl From<$name> for $foreign {
            fn from(value: $name) -> Self {
                Self {
                    $($field_name: value.$field_name,)*
                    ..Default::default()
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                <$foreign>::default().into()
            }
        }
    };
}

pub(super) use enum_def;
pub(super) use options_def;
