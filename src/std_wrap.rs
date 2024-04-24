use crate::{std_eq, std_ord, std_partial_eq, std_partial_ord, std_wrap_struct};

std_wrap_struct! { StdWrap }
std_wrap_struct! { StdWrap2 <T> T }
std_wrap_struct! { [Clone, Debug] StdWrap3 <T> t T }

std_partial_eq! { StdWrap <T> T }
std_eq! { StdWrap <T> T }
std_partial_ord! { StdWrap <T> T }
std_ord! { StdWrap <T> T }
