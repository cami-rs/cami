std_wrap! { StdWrap }
std_wrap! { StdWrap2 <T> T }
std_wrap! { [Clone, Debug] StdWrap3 <T> t T }

std_partial_eq! { StdWrap <T> T }
std_eq! { StdWrap <T> T }
std_partial_ord! { StdWrap <T> T }
std_ord! { StdWrap <T> T }
