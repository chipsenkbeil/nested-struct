# Nested Struct

Ever wanted to be able to create a data structure that contains nested
data? Do you find yourself creating many individual structs simply to
combine them together? Well, this is the library for you!

Transform this

```rust
pub struct MyStruct {
    pub data: MyStructData,
}

pub struct MyStructData {
    pub data: u32
}
```

Into this

```rust
use nested_struct::*;

nested_struct! {
    pub struct MyStruct {
        pub data: MyStructData {
            pub data: u32
        }
    }
}
```

### Basic usage

Creates a new struct that may feature nested fields.

```rust
use nested_struct::*;

nested_struct! {
    pub struct MyStruct {
        pub regular_field: u32,
        pub nested_field: NestedField {
            pub inner_field: bool
        }
    }
}

let _ = MyStruct {
    regular_field: 123,
    nested_field: NestedField {
        inner_field: true,
    },
};
```

### Deeply-nested structs

Nesting is not limited to a single level. You can generate structs with multi-nested fields:

```rust
use nested_struct::*;

nested_struct! {
    pub struct MyStruct {
        pub nested_field: NestedField {
            pub nested_field: DeeperNestedField {
                pub inner_field: bool
            }
        }
    }
}

let _ = MyStruct {
    nested_field: NestedField {
        nested_field: DeeperNestedField {
            inner_field: true,
        }
    },
};
```

### Applying field attributes to fields that are nested structs

Like with a normal struct, nested fields can have attributes placed on them:

```rust
use nested_struct::*;

nested_struct! {
    pub struct MyStruct {
        pub regular_field: u32,

        #[doc = "my nested field"]
        pub nested_field: NestedField {
            pub inner_field: bool
        }
    }
}
```

### Applying struct-level attributes to nested structs

If you want to apply attributes on the generated, nested struct, you need to
use the `@nested` marker. This can be used multiple times, but must occur AFTER
any field-specific attributes:

```rust
use nested_struct::*;

nested_struct! {
    pub struct MyStruct {
        pub regular_field: u32,

        #[doc = "my nested field"]
        @nested(#[derive(Clone)])
        pub nested_field: NestedField {
            pub inner_field: bool
        }
    }
}

let nested_field = NestedField { inner_field: true };
let _ = MyStruct {
    regular_field: 123,
    nested_field: nested_field.clone(),
};
```

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
