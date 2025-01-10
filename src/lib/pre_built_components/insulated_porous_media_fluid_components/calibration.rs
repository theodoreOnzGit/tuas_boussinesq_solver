use super::InsulatedPorousMediaFluidComponent;
use uom::si::f64::*;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;

use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;


impl InsulatedPorousMediaFluidComponent {
    
    /// calibrates the insulation thickness of this pipe or component, 
    /// to increase or decrease parasitic heat loss
    /// however, will not change thermal inertia
    /// 
    pub fn calibrate_insulation_thickness(&mut self, 
        pipe_length: Length,
        insulation_id: Length,
        insulation_thickness: Length){

        let insulation_od = insulation_id + 2.0*insulation_thickness.abs();
        let insulation_mid_diameter: Length = (insulation_od + insulation_id)/2.0;

        // this insulation thermal conductivity is arbitrary
        // we find conductance, and divide by the same 
        // thermal conductivity to find the thermal 
        // conductance lengthscale
        let arbitrary_insulation_thermal_conductivity: ThermalConductivity = 
            ThermalConductivity::new::<watt_per_meter_kelvin>(1.0);

        let insulation_conductance_to_ambient: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                insulation_mid_diameter,
                insulation_od,
                pipe_length,
                arbitrary_insulation_thermal_conductivity).unwrap();


        let insulation_conductance_to_pipe_insulation_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                insulation_id,
                insulation_mid_diameter,
                pipe_length,
                arbitrary_insulation_thermal_conductivity).unwrap();

        self.thermal_conductance_lengthscale_insulation_to_ambient = 
            insulation_conductance_to_ambient/arbitrary_insulation_thermal_conductivity;

        self.thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface = 
            insulation_conductance_to_pipe_insulation_boundary/arbitrary_insulation_thermal_conductivity;

        
    }
}
