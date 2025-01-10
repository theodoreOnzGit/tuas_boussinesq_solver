
/// this ensures that despite all the changes, the three branch 
/// ciet should still reproduce results 
///
/// eg. natural circulation loop
///
#[cfg(test)]
pub mod regression_tests;


/// this function runs ciet ver 1 test, 
/// mass flowrates are calculated serially
/// for simplicity
///
/// it works well enough, but the CTAH branch controller 
/// is not configured yet
#[cfg(test)]
pub mod version_1;

#[cfg(test)]
pub use version_1::*;

/// version 2 adds CTAH control capabilities 
/// to the simulator 
///
/// It runs in a single threaded manner. 
///
#[cfg(test)]
pub mod version_2;

/// version 3 adds parallelism to version 2,
/// hopefully to get it faster
pub mod version_3;
