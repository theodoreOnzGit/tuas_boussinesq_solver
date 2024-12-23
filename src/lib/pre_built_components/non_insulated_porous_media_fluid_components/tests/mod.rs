/// heater with struct supports, mx 10, and top and bottom head,
/// has a csv writer for results
#[cfg(test)]
mod heater_example_with_data_record;

/// this does not have csv writer, was used for early test and development
#[cfg(test)]
mod heated_section_only;

/// basically, I updated the heater v2 class to be a more generic 
/// porous media pipe without insulation. I just want to make sure 
/// this is working properly in terms of lateral connections and 
/// advancing timesteps etc. 
///
/// the old heater code and new heater code should perform the same 
/// way
#[cfg(test)]
mod heated_section_regression;

/// this does not have csv writer, was used for early test and development
#[cfg(test)]
mod heated_section_with_top_bottom_heads;

/// this does not have csv writer, was used for early test and development
#[cfg(test)]
mod heated_section_with_top_bottom_heads_and_mx10;

/// this does not have csv writer, was used for early test and development
#[cfg(test)]
mod heated_section_with_top_bottom_heads_and_mx10_and_struct_supports;
