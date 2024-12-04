
/// this ensures that despite all the changes, the three branch 
/// ciet should still reproduce results 
///
/// eg. natural circulation loop
///
pub mod regression_tests;


/// this function runs ciet ver 1 test, 
/// mass flowrates are calculated serially
/// for simplicity
///
/// it may work, but this runs slower than real-time
pub mod version_1;

#[cfg(test)]
pub use version_1::*;


/// version 2 is version 1 with speed enhancements 
/// basically a better algorithm flow and 
/// some parallel computing 
pub mod version_2;

#[cfg(test)]
pub use version_2::*;
