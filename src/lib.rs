#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! nested_struct {
    // [MAIN] Primary rule to generate the struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $(@nested(#[$field_nested_meta:meta]))*
                $field_vis:vis $field_name:ident : $field_ty:ident $({
                    $($field_ty_inner:tt)*
                })?
            ),*
        $(,)? }
    ) => {
        // Generate our primary struct
        $(#[$meta])* $vis struct $name {
            $(
                $(#[$field_meta])*
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
    #[allow(dead_code)]
    fn supports_named_struct_with_regular_fields() {
        nested_struct! {
            struct TestStruct {
                field: u32
            }
        }

        let _ = TestStruct { field: 123 };
    }

    #[test]
    #[allow(dead_code)]
    fn supports_named_struct_with_nested_fields() {
        nested_struct! {
            struct TestStruct {
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
    #[allow(dead_code)]
    fn supports_named_struct_with_deeply_nested_fields() {
        nested_struct! {
            struct TestStruct {
                field: NestedField {
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

    #[test]
    #[allow(dead_code)]
    fn supports_named_struct_with_nested_fields_using_nested_attribute() {
        // Single nested version
        {
            nested_struct! {
                #[derive(Clone)]
                struct TestStruct {
                    @nested(#[derive(Clone)])
                    field: NestedField {
                        field: u32
                    }
                }
            }

            let _ = TestStruct {
                field: NestedField { field: 123 },
            };
        }

        // Deeply nested version
        {
            nested_struct! {
                #[derive(Clone)]
                struct TestStruct {
                    @nested(#[derive(Clone)])
                    field: NestedField {
                        @nested(#[derive(Clone)])
                        field: NestedField2 {
                            field: u32
                        }
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
}
