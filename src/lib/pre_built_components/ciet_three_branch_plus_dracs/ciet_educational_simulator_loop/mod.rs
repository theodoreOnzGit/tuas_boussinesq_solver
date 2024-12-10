
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
/// it works well enough, but the CTAH branch is not configured yet
pub mod version_1;

#[cfg(test)]
pub use version_1::*;
