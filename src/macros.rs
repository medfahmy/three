/// Implements the following traits:
///
/// * `AsRef<object::Base>`
/// * `AsMut<object::Base>`
/// * `Object`
///
/// # Examples
///
/// Creating a wrapper around a named field.
///
/// ```rust
/// #[macro_use]
/// three_object!(MyStruct::mesh);
/// struct MyStruct {
///     mesh: three::Mesh,
/// }
/// # fn main() {}
/// ```
///
/// If the field parameter is omitted then the field name defaults to `object`.
///
/// ```rust
/// #[macro_use]
/// // Equivalent to `three_object!(MyStruct::object);`
/// three_object!(MyStruct);
/// struct MyStruct {
///     object: three::Mesh,
/// }
/// # fn main() {}
/// ```
///
/// [`object::Base`]: object/struct.Base.html
#[macro_export]
macro_rules! three_object {
    ($name:ident::$field:ident) => {
        impl AsRef<$crate::object::Base> for $name {
            fn as_ref(&self) -> &$crate::object::Base {
                &self.$field.as_ref()
            }
        }

        impl $crate::Object for $name {
            type Data = ();

            fn resolve_data(&self, _: &$crate::scene::SyncGuard) -> Self::Data {}
        }
    };

    ($name:ident) => {
        three_object!($name::object);
    };
}

// macro_rules! derive_DowncastObject {
//     ($type:ident => $pattern:path) => {
//         impl ::object::DowncastObject for $type {
//             fn downcast(object: ::object::ObjectType) -> Option<Self> {
//                 match object {
//                     $pattern(inner) => Some(inner),
//                     _ => None,
//                 }
//             }
//         }
//     };
// }
