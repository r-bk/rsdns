macro_rules! get_bit {
    ($e:expr, $l:literal) => {
        ($e & (1 << $l)) != 0
    };
}

macro_rules! set_bit {
    ($e:expr, $l:literal, $v:ident) => {
        let mask = 1 << $l;
        if $v {
            $e |= mask;
        } else {
            $e &= !mask;
        }
    };
}
