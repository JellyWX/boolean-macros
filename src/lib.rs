#[macro_export]
macro_rules! not {
    ( $x:expr ) => {
        !$x
    };
}

#[macro_export]
macro_rules! and {
    ( $e:expr ) => {
        $e
    };
    ( $left:expr, $( $rest:expr ),+ ) => {
        $left && and!($( $rest ),+)
    };
}

#[macro_export]
macro_rules! or {
    ( $e:expr ) => {
        $e
    };
    ( $left:expr, $( $rest:expr ),+ ) => {
        $left || or!($( $rest ),+)
    };
}

#[macro_export]
macro_rules! xor {
    ( $e:expr ) => {
        $e
    };
    ( $left:expr, $right:expr ) => {
        ($left || $right) && !($left && $right)
    };
    ( $left:expr, $mid:expr, $( $rest:expr ),+ ) => {
        xor!(xor!($left, $mid), $( $rest ),+)
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn not_macro() {
        assert!(not!(false));
        assert!(!not!(true));
    }

    #[test]
    fn and_macro() {
        assert!(and!(true));
        assert!(!and!(false));
        assert!(and!(true, true));
        assert!(!and!(false, true));
        assert!(and!(true, true, true));
        assert!(!and!(true, true, false));
    }

    #[test]
    fn or_macro() {
        assert!(or!(true, true, true));
        assert!(or!(true, true));
        assert!(or!(true));
        assert!(!or!(false));
        assert!(or!(false, true));
        assert!(or!(true, false, true));
    }

    #[test]
    fn xor_macro() {
        assert!(xor!(true, false));
        assert!(xor!(false, true));

        assert!(!xor!(true, true));
        assert!(!xor!(false, false));

        assert!(xor!(true, false, false));
        assert!(!xor!(true, true, false));
        assert!(xor!(true, true, true));
    }
}
