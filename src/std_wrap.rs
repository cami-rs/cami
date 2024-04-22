use crate::{s_eq, s_ord, s_partial_eq, s_partial_ord, s_wrap};

s_wrap! { StdWrap }
s_wrap! { StdWrap2 <T> T }
s_wrap! { [Clone, Debug] StdWrap3 <T> t T }

s_partial_eq! { StdWrap <T> T }
s_eq! { StdWrap <T> T }
s_partial_ord! { StdWrap <T> T }
s_ord! { StdWrap <T> T }
