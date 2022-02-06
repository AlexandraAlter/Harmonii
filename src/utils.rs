macro_rules! some_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
    ( $e:expr, $r:expr ) => {
        match $e {
            Some(x) => x,
            None => {
                $r();
                return;
            },
        }
    };
}

macro_rules! result_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
    ( $e:expr, $r:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $r(e);
                return;
            },
        }
    };
}
