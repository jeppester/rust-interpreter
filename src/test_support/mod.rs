#![macro_use]
macro_rules! match_or_fail {
  ($expression:expr, $matcher:pat => $result:expr) => {
    match $expression {
      $matcher => $result,
      ref e => panic!("Expected match for:\n\t{}\nGot:\n\t{:?}", stringify!($matcher), e),
    }
  }
}

