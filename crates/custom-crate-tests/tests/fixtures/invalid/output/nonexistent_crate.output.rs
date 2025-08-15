#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::nonexistent_crate::TypedUuidKind for UserKind {
    #[inline]
    fn tag() -> ::nonexistent_crate::TypedUuidTag {
        const TAG: ::nonexistent_crate::TypedUuidTag = ::nonexistent_crate::TypedUuidTag::new(
            "user",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserUuid))
    }
}
#[allow(unused)]
pub type UserUuid = ::nonexistent_crate::TypedUuid<UserKind>;
