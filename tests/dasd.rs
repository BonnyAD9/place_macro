fn asd() {
    macro_rules! csi_macro {
        ($(
            $name:ident
            $(, $($nam:ident)? $($lit:literal)?)+ ;
            $i:literal $(?$doc:literal)?),+ $(,)?
        ) => {
            place_macro::place! {$(
                $(#[doc = __repnl__($doc, " ")])?
                #[macro_export]
                macro_rules! $name {
                    ($($(__s__ $nam:expr)?),+) => {
                        __s__ crate::csi!($i, $($(__s__ $nam)? $($lit)?),+)
                    }
                }
            )+}
        };
    }

    csi_macro!(
        move_up, n; 'A' ? "Moves cursor up by N positions",
        move_down, n; 'B' ? "Moves cursor up by N positions",
        move_right, n; 'C' ? "Moves cursor up by N positions",
        move_left, n; 'D' ? "Moves cursor up by N positions",
        set_down, n; 'E' ? "Moves cursor to the start of line N lines down",
        set_up, n; 'E' ? "Moves cursor to the start of line N lines up",
        column, n; 'G' ? "Moves cursor to the given column",
    );
}
