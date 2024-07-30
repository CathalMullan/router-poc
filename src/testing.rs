#[macro_export]
macro_rules! assert_router_matches {
    ($router:expr, {
        $( $input:expr => $expected:tt )+
    }) => {
        $(
            match $router.matches($input) {
                #[allow(unused_variables)]
                Some($crate::matches::Match { data, parameters }) => {
                    assert_router_matches!(@match_found $input, data, parameters, $expected);
                }
                None => {
                    assert_router_matches!(@no_match $input, $expected);
                }
            }
        )+
    };

    (@match_found $input:expr, $data:ident, $parameters:ident, {
        path: $path:expr,
        value: $value:expr
        $(, params: {
            $($param_key:expr => $param_value:expr),+
        })?
    }) => {
        assert_eq!($data.path, $path);
        assert_eq!($data.value, $value);

        $(
            let expected_params: smallvec::SmallVec<[$crate::matches::Parameter<'_>; 4]> = smallvec::smallvec![
                $( $crate::matches::Parameter {
                    key: $param_key.as_bytes(),
                    value: $param_value.as_bytes()
                } ),+
            ];

            assert_eq!($parameters, expected_params);
        )?
    };

    (@match_found $input:expr, $data:ident, $parameters:ident, None) => {
        panic!("Expected no match for input '{}'", $input);
    };

    (@no_match $input:expr, None) => {};

    (@no_match $input:expr, $expected:tt) => {
        panic!("No match found for input '{}'", $input);
    };
}
