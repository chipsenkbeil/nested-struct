/// Creates a new struct that may feature nested fields.
///
/// ```
/// # use nested_struct::nested_struct;
/// nested_struct! {
///     pub struct MyStruct {
///         pub regular_field: u32,
///         pub nested_field: NestedField {
///             pub inner_field: bool
///         }
///     }
/// }
///
/// let _ = MyStruct {
///     regular_field: 123,
///     nested_field: NestedField {
///         inner_field: true,
///     },
/// };
/// ```
///
/// ### Deeply-nested structs
///
/// Nesting is not limited to a single level. You can generate structs with multi-nested fields:
///
/// ```
/// # use nested_struct::nested_struct;
/// nested_struct! {
///     pub struct MyStruct {
///         pub nested_field: NestedField {
///             pub nested_field: DeeperNestedField {
///                 pub inner_field: bool
///             }
///         }
///     }
/// }
///
/// let _ = MyStruct {
///     nested_field: NestedField {
///         nested_field: DeeperNestedField {
///             inner_field: true,
///         }
///     },
/// };
/// ```
///
/// ### Applying field attributes to fields that are nested structs
///
/// Like with a normal struct, nested fields can have attributes placed on them:
///
/// ```
/// # use nested_struct::nested_struct;
/// nested_struct! {
///     pub struct MyStruct {
///         pub regular_field: u32,
///
///         #[doc = "my nested field"]
///         pub nested_field: NestedField {
///             pub inner_field: bool
///         }
///     }
/// }
/// ```
///
/// ### Applying struct-level attributes to nested structs
///
/// If you want to apply attributes on the generated, nested struct, you need to
/// use the `@nested` marker. This can be used multiple times, but must occur BEFORE
/// any field-specific attributes:
///
/// ```
/// # use nested_struct::nested_struct;
/// nested_struct! {
///     pub struct MyStruct {
///         pub regular_field: u32,
///
///         @nested(#[derive(Clone)])
///         #[doc = "my nested field"]
///         pub nested_field: NestedField {
///             pub inner_field: bool
///         }
///     }
/// }
///
/// let nested_field = NestedField { inner_field: true };
/// let _ = MyStruct {
///     regular_field: 123,
///     nested_field: nested_field.clone(),
/// };
/// ```
///
#[macro_export]
macro_rules! nested_struct {
    // Primary rule to generate the struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $( @nested(#[$field_nested_meta:meta]) )*
                $( #[$field_meta:meta] )*
                $field_vis:vis $field_name:ident : $field_ty:ident $({
                    $($field_ty_inner:tt)*
                })?
            ),*
        $(,)? }
    ) => {
        // Generate our primary struct
        $( #[$meta] )*
        $vis struct $name {
            $(
                $( #[$field_meta] )*
                $field_vis $field_name : $field_ty
            ),*
        }

        // Generate our inner structs for fields
        $(nested_struct! {
            @nested
            $(#[$field_nested_meta])*
            $field_vis $field_ty $({
                $($field_ty_inner)*
            })?
        })*
    };

    // [INCLUDE] Used to filter out struct generation to only nested types
    (@nested $(#[$meta:meta])* $vis:vis $name:ident {$($fields:tt)*}) => {
        nested_struct! {
            $(#[$meta])*
            $vis struct $name {
                $($fields)*
            }
        }
    };

    // [EXCLUDE] Used to filter out struct generation to only nested types
    (@nested $(#[$meta:meta])* $vis:vis $name:ident) => {};

    // Any garbage we will ignore, including generating an invalid struct
    /* ($($other:tt)*) => {
        compile_error!(stringify!($($other)*));
    }; */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_empty_named_struct() {
        nested_struct! {
            struct TestStruct {}
        }

        let _ = TestStruct {};
    }

    #[test]
    fn supports_named_struct_with_regular_fields() {
        nested_struct! {
            #[allow(dead_code)]
            struct TestStruct {
                field: u32
            }
        }

        let _ = TestStruct { field: 123 };
    }

    #[test]
    fn supports_named_struct_with_nested_fields() {
        nested_struct! {
            #[allow(dead_code)]
            struct TestStruct {
                @nested(#[allow(dead_code)])
                field: NestedField {
                    field: u32
                }
            }
        }

        let _ = TestStruct {
            field: NestedField { field: 123 },
        };
    }

    #[test]
    fn supports_named_struct_with_deeply_nested_fields() {
        nested_struct! {
            #[allow(dead_code)]
            struct TestStruct {
                @nested(#[allow(dead_code)])
                field: NestedField {
                    @nested(#[allow(dead_code)])
                    field: NestedField2 { field: u32 }
                }
            }
        }

        let _ = TestStruct {
            field: NestedField {
                field: NestedField2 { field: 123 },
            },
        };
    }
}
