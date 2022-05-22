pub use enum_const_derive::EnumConst;

/// example:
/// ```
/// use enum_const::EnumConst;
///
/// #[derive(EnumConst, PartialEq, Debug)]
/// enum Foo {
///     X = 1,
///     Y = 2,
///     Z,
/// }
/// #[test]
/// fn it_works() {
///     assert_eq!(Some(Foo::X), Foo::from_const_isize(1));
///     assert_eq!(Some(1isize), Foo::X.get_const_isize());
/// }
/// ```
pub trait EnumConst: Sized {
    fn get_const_isize(&self) -> Option<isize>;
    fn from_const_isize(i: isize) -> Option<Self>;
}
