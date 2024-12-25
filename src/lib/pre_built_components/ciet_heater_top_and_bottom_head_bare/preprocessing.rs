use std::f64::consts::PI;
use std::thread::{self};
use std::thread::JoinHandle;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::NusseltPrandtlReynoldsData;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use super::HeaterTopBottomHead;
use uom::si::f64::*;
use uom::si::length::{inch, meter};
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use ndarray::*;

impl HeaterTopBottomHead {


    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// executes serially, and uses lots of cloning, so it's 
    /// heavier in resource usage,
    ///
    /// unoptimised in this regard
    #[inline]
    pub fn lateral_and_miscellaneous_connections(&mut self,
        mass_flowrate: MassRate){

        let heater_steady_state_power = Power::ZERO;

        // first let's get all the conductances 

        let steel_to_air_conductance: ThermalConductance 
        = self.get_air_steel_shell_conductance(
            self.heat_transfer_to_air
        );

        self.set_mass_flowrate(mass_flowrate);

        let steel_surf_to_therminol_conductance: ThermalConductance 
        = self.get_therminol_node_steel_shell_conductance();

        let twisted_tape_to_therminol_conductance: ThermalConductance 
        = self.get_therminol_node_twisted_tape_conductance();

        // other stuff 
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let q_fraction_per_node: f64 = 1.0/ number_of_temperature_nodes as f64;
        let mut q_frac_arr: Array1<f64> = Array::default(number_of_temperature_nodes);
        q_frac_arr.fill(q_fraction_per_node);

        // then get the ambient temperature 

        let ambient_air_temp = self.ambient_temperature;

        // lateral connections 
        {
            // first i will need to create temperature vectors 

            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
            = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_air_temp);


            // clone each array and set them later

            let mut steel_shell_clone: SolidColumn = 
            self.steel_shell.clone().try_into() .unwrap();

            let mut therminol_array_clone: FluidArray = 
            self.therminol_array.clone().try_into().unwrap();

            let mut twisted_tape_array_clone: SolidColumn = 
            self.twisted_tape_interior.clone().try_into().unwrap();

            // note, must set mass flowrate first 
            // otherwise there is by default zero flow through 
            // the array

            therminol_array_clone.set_mass_flowrate(
                mass_flowrate);

            // temperature vectors

            let steel_temp_vector: Vec<ThermodynamicTemperature> 
            = steel_shell_clone.get_temperature_vector().unwrap();

            let fluid_temp_vector: Vec<ThermodynamicTemperature> 
            = therminol_array_clone.get_temperature_vector().unwrap();

            let twisted_tape_temp_vector: Vec<ThermodynamicTemperature> 
            = twisted_tape_array_clone.get_temperature_vector().unwrap();
            // second, fill them into the each array 
            
            // steel to air interaction

            steel_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
                steel_to_air_conductance,
                ambient_temperature_vector
            ).unwrap();

            // steel shell to therminol interaction

            steel_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
                steel_surf_to_therminol_conductance,
                fluid_temp_vector.clone()
            ).unwrap();

            therminol_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                steel_surf_to_therminol_conductance,
                steel_temp_vector
            ).unwrap();

            // we also want to add a heat source to steel shell

            steel_shell_clone.lateral_link_new_power_vector(
                heater_steady_state_power,
                q_frac_arr
            ).unwrap();

            // now therminol to twisted tape interaction

            therminol_array_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    twisted_tape_to_therminol_conductance,
                    twisted_tape_temp_vector).unwrap();

            twisted_tape_array_clone. 
                lateral_link_new_temperature_vector_avg_conductance(
                    twisted_tape_to_therminol_conductance,
                    fluid_temp_vector).unwrap();



            // now that lateral connections are done, 
            // modify the heat transfer entity 

            self.therminol_array.set(therminol_array_clone.into()).unwrap();

            self.steel_shell.set(steel_shell_clone.into()).unwrap();

            self.twisted_tape_interior.set(twisted_tape_array_clone.into()
                ).unwrap();


        }
        // axial connections 

        self.zero_power_bc_connection();
    }


    /// the end of each node should have a zero power boundary condition 
    /// connected to each of them at the bare minimum
    ///
    /// this function does exactly that
    ///
    /// to connect the rest of the heat transfer entities, 
    /// use the link to front or back methods within the 
    /// FluidArray or SolidColumn
    #[inline]
    fn zero_power_bc_connection(&mut self){

        let zero_power: Power = Power::ZERO;

        let mut zero_power_bc: HeatTransferEntity = 
        BCType::UserSpecifiedHeatAddition(zero_power).into();

        // constant heat addition interaction 

        let interaction: HeatTransferInteractionType = 
        HeatTransferInteractionType::UserSpecifiedHeatAddition;

        // now connect the twisted tape 

        self.twisted_tape_interior.link_to_back(&mut zero_power_bc,
            interaction).unwrap();


        self.twisted_tape_interior.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.therminol_array.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.therminol_array.link_to_back(&mut zero_power_bc,
            interaction).unwrap();

        self.steel_shell.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.steel_shell.link_to_back(&mut zero_power_bc,
            interaction).unwrap();
    }




    /// obtains air to steel shell conductance
    #[inline]
    pub fn get_air_steel_shell_conductance(&mut self,
    h_air_to_steel_surf: HeatTransfer) 
        -> ThermalConductance {
        // first, let's get a clone of the steel shell surface
        let mut steel_shell_clone: SolidColumn = 
        self.steel_shell.clone().try_into().unwrap();

        let mut therminol_array_clone: FluidArray = 
        self.therminol_array.clone().try_into().unwrap();

        let number_of_temperature_nodes = self.inner_nodes + 2;
        let component_length = therminol_array_clone.get_component_length();
        let hydraulic_diameter = therminol_array_clone.get_hydraulic_diameter();
        let id = Length::new::<meter>(0.0381);
        let od = Length::new::<meter>(0.04);

        let node_area: Area = PI *  hydraulic_diameter * component_length
        / number_of_temperature_nodes as f64;


        let air_to_surface_node_conductance: ThermalConductance = 
        h_air_to_steel_surf * node_area;

        // next is to have steel inner conductance

        let steel_surf_temperature: ThermodynamicTemperature 
        = steel_shell_clone.try_get_bulk_temperature().unwrap();

        let cylinder_mid_diameter: Length = 0.5*(id+od);

        let steel_conductivity: ThermalConductivity = 
        SolidMaterial::SteelSS304L.try_get_thermal_conductivity(
            steel_surf_temperature
        ).unwrap();

        let node_length = component_length / 
            number_of_temperature_nodes as f64;

        let cylinder_node_conductance: ThermalConductance 
        = try_get_thermal_conductance_annular_cylinder(
            cylinder_mid_diameter,
            od,
            node_length,
            steel_conductivity
        ).unwrap();

        let air_to_steel_resistance = 1.0/cylinder_node_conductance 
        + 1.0/air_to_surface_node_conductance;

        return 1.0/air_to_steel_resistance;
    }


    /// obtains therminol to steel shell conductance
    #[inline]
    pub fn get_therminol_node_steel_shell_conductance(&mut self) 
        -> ThermalConductance {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the therminol fluid array and twisted tape array
        let mut therminol_fluid_array_clone: FluidArray = 
        self.therminol_array.clone().try_into().unwrap();

        let mut steel_shell_clone: SolidColumn = 
        self.steel_shell.clone().try_into().unwrap();

        // also need to get basic tmeperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let mass_flowrate: MassRate = 
        therminol_fluid_array_clone.get_mass_flowrate();

        let bulk_temperature: ThermodynamicTemperature 
        = therminol_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let steel_surf_temperature: ThermodynamicTemperature 
        = steel_shell_clone.try_get_bulk_temperature().unwrap();

        let hydraulic_diameter = 
        therminol_fluid_array_clone.get_hydraulic_diameter();

        // firstly, reynolds 

        let reynolds_number: Ratio = 
        self.heater_v2_hydraulic_diameter_reynolds(
            mass_flowrate,
            bulk_temperature,
        );

        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
        = LiquidMaterial::TherminolVP1.try_get_prandtl_liquid(
            bulk_temperature,
            atmospheric_pressure
        ).unwrap();


        // for this case, I will have the ciet heater nusselt 
        // number correlation
        //
        // constants are ignored, so we use the default method
        // and manually adjust the reynolds and prandtl numbers

        let mut heater_prandtl_reynolds_data: NusseltPrandtlReynoldsData 
        = NusseltPrandtlReynoldsData::default();

        heater_prandtl_reynolds_data.reynolds = reynolds_number;
        heater_prandtl_reynolds_data.prandtl_bulk = bulk_prandtl_number;
        heater_prandtl_reynolds_data.prandtl_wall = bulk_prandtl_number;

        let heater_nusselt_correlation: NusseltCorrelation 
        =  NusseltCorrelation::CIETHeaterVersion2(
            heater_prandtl_reynolds_data
        );

        let nusselt_estimate: Ratio = 
        heater_nusselt_correlation.try_get_nusselt().unwrap();

        // now we can get the heat transfer coeff, 

        let h: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
        LiquidMaterial::TherminolVP1.try_get_thermal_conductivity(
            bulk_temperature).unwrap();

        h = nusselt_estimate * k_fluid_average / hydraulic_diameter;

        // and then get the convective resistance
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = Length::new::<meter>(1.6383);
        let heated_length_plus_heads = Length::new::<inch>(78.0);
        let id = Length::new::<meter>(0.0381);
        let od = Length::new::<meter>(0.04);

        let heat_transfer_area_heated_length_plus_heads: Area = 
        heated_length_plus_heads* id * PI;

        let heat_transfer_area_heated_length_only: Area
        = heated_length/ heated_length_plus_heads * 
        heat_transfer_area_heated_length_plus_heads;


        let heat_transfer_area_per_node: Area 
        = heat_transfer_area_heated_length_only / 
        number_of_temperature_nodes as f64;

        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        let therminol_to_steel_shell_average_conductance: ThermalConductance 
        = h * heat_transfer_area_per_node;

        let therminol_to_steel_shell_surface_node_resistance = 
        1.0/therminol_to_steel_shell_average_conductance;

        // now I need to calculate resistance of the half length of the 
        // steel shell, which is an annular cylinder

        let cylinder_mid_diameter: Length = 0.5*(id+od);

        let steel_conductivity: ThermalConductivity = 
        SolidMaterial::SteelSS304L.try_get_thermal_conductivity(
            steel_surf_temperature
        ).unwrap();

        let cylinder_node_conductance: ThermalConductance 
        = try_get_thermal_conductance_annular_cylinder(
            id,
            cylinder_mid_diameter,
            node_length,
            steel_conductivity
        ).unwrap();


        let cylinder_node_resistance = 
        1.0/cylinder_node_conductance;

        let cylinder_to_therminol_resistance = 
        cylinder_node_resistance + therminol_to_steel_shell_surface_node_resistance;

        let cylinder_to_therminol_conductance: ThermalConductance 
        = 1.0/cylinder_to_therminol_resistance;

        return cylinder_to_therminol_conductance;
    }

    /// gets reynolds number based on top and bottom head 
    /// hydraulic diameter and cross sectional flow area
    ///
    /// the user must supply mass flowrate and temperature
    #[inline]
    pub fn heater_v2_hydraulic_diameter_reynolds(&self,
        mass_flowrate: MassRate,
        temperature: ThermodynamicTemperature) -> Ratio {

        let mut therminol_fluid_array_clone: FluidArray = 
        self.therminol_array.clone().try_into().unwrap();


        let flow_area: Area = 
        therminol_fluid_array_clone.get_cross_sectional_area();
        let hydraulic_diameter = 
        therminol_fluid_array_clone.get_hydraulic_diameter();
        let viscosity: DynamicViscosity = 
        LiquidMaterial::TherminolVP1.try_get_dynamic_viscosity(
            temperature).unwrap();

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds: Ratio = 
        mass_flowrate/flow_area*hydraulic_diameter / viscosity;

        reynolds

    }


    /// obtains therminol to twisted tape conductance 
    /// based on approx wakao correlation
    #[inline]
    pub fn get_therminol_node_twisted_tape_conductance(
    &self) -> ThermalConductance {

        // the twisted tape itself acts as a thermal 
        // mass and effectively the twisted tape in the 
        // heater acts like porous media
        //
        // I need to find the nusselt correlation 
        // (otherwise use the Wakao Correlation) 
        //
        // and I also need the mass of the twisted tape overall 
        //
        // From De Wet's Dissertation, 
        // The heat transfer area for the twisted tape is 
        // 719 in^2 
        //
        // Also, the volume fraction of fluid in the 
        // original heater as compared to the 
        // fluid in the entire loop was approximately 3\% 
        // with heater v1.0, now, 
        //
        // There were two inserts tested in Lukas's 
        // conference paper 
        // First, a 51\% open perforated insert 
        // and a 23\% open one
        //
        // The 51\% open insert was used for heater v2.0 
        //
        // compared to the annular tube, it had a 157\%  
        // increase in residence time. Or for a constant 
        // mass or volumetric flow rate, a 157\% increase in volume 
        //
        // also, the volume fraction increased from 3\% of the 
        // loop to 8.1\% of the loop
        //
        // For heater v1.0, 
        // the flow volume is about 42.12 in^3 (the fluid volume height 
        // is 198 cm which includes the heater top and bottom heads)
        // whereas the flow volume in heater v2.0 is about 127 in^3
        //
        // Taking heater outer tube inner diameter of 1.5 in
        // and height of 78 in 
        // we get flow volume of 137.83 in^3
        //
        // Which means the twisted tape plus perforrated tube is about 
        // 10 in^3 of steel. We can use this to estimate the 
        // thermal inertia...
        //
        // I'm not too sure about the 157\% increase in residence 
        // time, 
        //
        // Okay, from Lukas's paper, the volume fraction of the 
        // fluid within the core compared to the loop increased from 
        // 3.3% to 8.1%, so this is about a 2.45 times as much as before 
        // assuming loop volume is reasonably large, this is close 
        // enough to the 3 times volume increase using the main fluid 
        // volume as a reference point 
        //
        // Hence, using the main fluid volume is right. And I think 
        // 10 in^3 of steel is reasonable for a thermal inertia 
        // measurement
        //
        // Thus fluid volume as modelled increases about 3 times
        // from v1.0 to v2.0, I wonder why we only have a 157\% increase 
        // in residence time. did the mass flowrate change?
        //
        // Apparently, the twisted tape height in fluid is 198cm 
        // which extends beyond the heated sections
        // 
        // So, heat transfer area is 719 in^2 including heater heads 
        // and the twisted tube is about 78 in or 198 cm long, which 
        // includes both heater heads, 
        //
        // we can scale heat transfer area accordingly using heated 
        // length of about 163 cm
        // 
        // for nusselt number, it seems best to use the Wakao 
        // Correlation as that is suitable for pebble beds anyhow
        // it's a best estimate, not need to be perfect for now 
        //
        // Can't really do much until someone does a separate 
        // effects test (SET)
        // 

        // find suitable heat transfer area
        let mut therminol_fluid_array_clone: FluidArray = 
        self.therminol_array.clone().try_into().unwrap();

        let component_length = 
        therminol_fluid_array_clone.get_component_length();




        // before any calculations, I will first need a clone of 
        // the therminol fluid array and twisted tape array
        let mut therminol_fluid_array_clone: FluidArray = 
        self.therminol_array.clone().try_into().unwrap();

        let mut twisted_tape_array_clone: SolidColumn = 
        self.twisted_tape_interior.clone().try_into().unwrap();
        // next, need the nusselt number based on Wakao Correlation 
        let mass_flowrate = therminol_fluid_array_clone.get_mass_flowrate();
        let flow_area: Area = therminol_fluid_array_clone.get_cross_sectional_area();
        let viscosity = therminol_fluid_array_clone.get_fluid_viscosity();
        let hydraulic_diameter = therminol_fluid_array_clone.get_hydraulic_diameter();

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds: Ratio = 
        mass_flowrate/flow_area*hydraulic_diameter / viscosity;

        // need to get prandtl number of fluid 
        // so I need fluid temperature 

        let fluid_average_temperature: ThermodynamicTemperature 
        = therminol_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let fluid_average_pressure: Pressure 
        = therminol_fluid_array_clone.pressure_control_volume;

        let fluid_material: LiquidMaterial = 
        therminol_fluid_array_clone.material_control_volume.try_into().unwrap();

        let fluid_prandtl: Ratio = fluid_material.try_get_prandtl_liquid
            (fluid_average_temperature, fluid_average_pressure) .unwrap();

        let twisted_tape_temperature: ThermodynamicTemperature 
        = twisted_tape_array_clone.try_get_bulk_temperature().unwrap();

        let twisted_tape_wall_prandtl: Ratio = 
        fluid_material.try_get_prandtl_liquid(
            twisted_tape_temperature, fluid_average_pressure).unwrap();

        // with Pr and Re, get nusselt estimate 
        //

        let nusselt_estimate: Ratio = 
        therminol_fluid_array_clone.get_nusselt(
            reynolds,
            fluid_prandtl,
            twisted_tape_wall_prandtl,
        ).unwrap();

        // with nusselt estimate done, (I didn't convert the 
        // hydraulic diameter to an equivalent particle diameter)
        // Now I can get a heat transfer coeff 
        //
        // conductance is that times the area


        let h: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
        fluid_material.try_get_thermal_conductivity(
            fluid_average_temperature).unwrap();

        h = nusselt_estimate * k_fluid_average / hydraulic_diameter;

        let number_of_temperature_nodes = self.inner_nodes + 2;

        let heat_transfer_area_per_node: Area 
        = PI * hydraulic_diameter * component_length / 
        number_of_temperature_nodes as f64;

        let average_node_conductance: ThermalConductance 
        = h * heat_transfer_area_per_node;

        return average_node_conductance;
    }

    /// spawns a thread and moves the entire heater object into the 
    /// thread, "locking" it for parallel computation
    ///
    /// once that is done, the join handle is returned 
    /// which when unwrapped, returns the heater object
    pub fn lateral_connection_thread_spawn(&self,
    mass_flowrate: MassRate) -> JoinHandle<Self>{

        // make an Arc Ptr 
        let mut heater_peripherals_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {

                // carry out the connection calculations
                heater_peripherals_clone.
                    lateral_and_miscellaneous_connections(
                        mass_flowrate);
                
                heater_peripherals_clone

            }
        );

        return join_handle;

    }
}


