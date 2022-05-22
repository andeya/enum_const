#[cfg(test)]
mod tests {
    use enum_const::EnumConst;

    #[derive(EnumConst, PartialEq, Debug)]
    enum Foo {
        X = 1,
        Y = 2,
        Z,
    }

    #[test]
    fn it_works() {
        assert_eq!(Some(Foo::X), Foo::from_const_isize(1));
        assert_eq!(Some(1isize), Foo::X.get_const_isize());
        assert_eq!(None, Foo::from_const_isize(3));
        assert_eq!(None, Foo::Z.get_const_isize());
    }
}
