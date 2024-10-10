use crate::boussinesq_thermophysical_properties::density::try_get_rho;
use crate::boussinesq_thermophysical_properties::specific_heat_capacity::try_get_cp;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::try_get_kappa_thermal_conductivity;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;
use crate::boussinesq_thermophysical_properties::Material;
use crate::heat_transfer_correlations::heat_transfer_interactions::advection_heat_rate;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use crate::heat_transfer_correlations::
heat_transfer_interactions::heat_transfer_interaction_enums::DataAdvection;
use crate::heat_transfer_correlations::
heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::
specific_enthalpy::try_get_temperature_from_h;

use super::SingleCVNode;
use uom::si::f64::*;
use uom::si::length::meter;
use uom::si::thermodynamic_temperature::kelvin;
use uom::num_traits::Zero;

impl SingleCVNode {


    /// this function calculates the conductance interaction between this 
    /// control volume and another control volume
    ///
    /// This excludes advection interactions
    #[inline]
    pub fn calculate_conductance_interaction_to_front_singular_cv_node(
        &mut self,
        single_cv_2: &mut SingleCVNode,
        interaction: HeatTransferInteractionType)-> Result<(), ThermalHydraulicsLibError>{

        // let's get the two temperatures of the control volumes first
        // so let me get the enthalpies, and then their respective 
        // temperatures 

        let single_cv_1_enthalpy = self.
            current_timestep_control_volume_specific_enthalpy;
        let single_cv_2_enthalpy = single_cv_2.
            current_timestep_control_volume_specific_enthalpy;

        // to get the temperatures, we'll need the material as well 
        let single_cv_1_material = self.material_control_volume;
        let single_cv_2_material = single_cv_2.material_control_volume;

        // we'll also need to get their pressures 
        let single_cv_1_pressure = self.pressure_control_volume;
        let single_cv_2_pressure = single_cv_2.pressure_control_volume;

        // we will now get their respective temperatures 
        //
        // (note, this is extremely computationally expensive as it 
        // is iterative in nature)
        //
        // two solutions here, 
        // one: store cv temperature in single cv, 
        // so it can be readily accessed
        //
        // two: cheaper method of getting t from h.


        let single_cv_1_temperature: ThermodynamicTemperature;
        let single_cv_2_temperature: ThermodynamicTemperature;

        let experimental_code = true; 
        if experimental_code {

            single_cv_1_temperature = self.temperature;
            single_cv_2_temperature = single_cv_2.temperature;

        } else {

            // original code
            single_cv_1_temperature = try_get_temperature_from_h(
                single_cv_1_material, 
                single_cv_1_enthalpy, 
                single_cv_1_pressure)?;
            single_cv_2_temperature = try_get_temperature_from_h(
                single_cv_2_material, 
                single_cv_2_enthalpy, 
                single_cv_2_pressure)?;

        }

        // now that we got their respective temperatures we can calculate 
        // the thermal conductance between them
        //
        // for conduction for instance, q = kA dT/dx 
        // conductance is watts per kelvin or 
        // q = (kA)/dx * dT
        // conductance here is kA/dx
        // thermal resistance is 1/conductance
        //
        // for convection, we get: 
        // q = hA (Delta T)
        // hA becomes the thermal conductance
        //
        // If we denote thermal conductance as Htc
        // 
        // Then a general formula for heat flowing from 
        // temperature T_1 to T_2 is 
        //
        // T_1 --> q --> T_2 
        //
        // q = - Htc (T_2 - T_1)

        // 
        let thermal_conductance = interaction.get_thermal_conductance_based_on_interaction(
            single_cv_1_temperature, 
            single_cv_2_temperature,
            single_cv_1_pressure, 
            single_cv_2_pressure)?;

        // suppose now we have thermal conductance, we can now obtain the 
        // power flow
        //

        let cv_2_temp_minus_cv_1_temp_kelvin: f64 = 
            single_cv_2_temperature.get::<kelvin>() - 
            single_cv_1_temperature.get::<kelvin>();

        let cv_2_temp_minus_cv_1: TemperatureInterval = 
            TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                cv_2_temp_minus_cv_1_temp_kelvin);

        let heat_flowrate_from_cv_1_to_cv_2: Power = 
            - thermal_conductance * cv_2_temp_minus_cv_1;

        // now, we add a heat loss term to cv_1 
        // and a heat gain term to cv_2 
        //
        // using timestep
        // the signs should cancel out

        self.rate_enthalpy_change_vector.
            push(-heat_flowrate_from_cv_1_to_cv_2);
        single_cv_2.rate_enthalpy_change_vector.
            push(heat_flowrate_from_cv_1_to_cv_2);


        // for solids mesh fourier number need only 
        // be done once, not every time 
        // an interaction is formed 
        //
        // probably the cell stability fourier number will be done in the 
        // constructor. however, with convection, the time scale must be 
        // recalculated at every time step. so it really depends whether 
        // it's solid or fluid control volume
        // Actually, for solid CV, I will also need to recalculate time scale 
        // based on the material thermal thermal_diffusivity
        //
        // For liquid CV, still need to calculate time scale based 
        // on convection flow
        // match statement is meant to tell that liquid CVs are not quite 
        // ready for use
        //
        // but the liquid timescales are calculated at the cv level only 
        // after all volumetric flowrates are calculated
        //
        // so don't really need any new timescale calculations

        return Ok(());

    }

    /// now, advection is quite tricky because the 
    /// heat transfer formula is for two control volumes cv_a and cv_b
    /// can be as follows 
    ///
    /// (cv_a) --------------- (cv_b)
    ///  
    ///  T_a                    T_b 
    ///
    ///  Q_(ab) = -H(T_b - T_a)
    ///
    ///  in this context, volume a is the current (self) control volume 
    ///  and b is the cv_2 (other control volume). The other control volume 
    ///  is placed in front of this control volume
    /// 
    /// Q_(ab) is heat transfer rate (watts) from a to b
    /// H is conductance, not heat transfer coefficient
    /// it has units of watts kelvin
    ///
    /// For advection in contrast, it depends on flow
    /// 
    /// (cv_a) --------------- (cv_b)
    ///  
    ///  T_a                    T_b 
    ///
    /// For flow from a to b:
    /// Q_(ab) = m h(T_a)
    ///
    /// For flow from b to a 
    /// Q_(ab) = -m h(T_b)
    ///
    /// Here, the enthalpy transfer only depends on one of the body's 
    /// temperature, which is directly dependent on mass flow 
    ///
    /// 
    #[inline]
    pub fn calculate_advection_interaction_to_front_singular_cv_node(
        &mut self,
        single_cv_2: &mut SingleCVNode,
        advection_data: DataAdvection)-> Result<(), ThermalHydraulicsLibError>{

        let mass_flow_from_cv_1_to_cv_2 = advection_data.mass_flowrate;

        // for this, quite straightforward, 
        // get both specific enthalpy of both cvs 
        // calculate power flow 
        // and then update the power vector 
        //

        let specific_enthalpy_cv1: AvailableEnergy = 
            self.current_timestep_control_volume_specific_enthalpy;

        let specific_enthalpy_cv2: AvailableEnergy = 
            single_cv_2.current_timestep_control_volume_specific_enthalpy;

        // calculate heat rate 

        let heat_flowrate_from_cv_1_to_cv_2: Power 
            = advection_heat_rate(mass_flow_from_cv_1_to_cv_2,
                specific_enthalpy_cv1,
                specific_enthalpy_cv2,)?;

        // by default, cv 1 is on the left, cv2 is on the right 
        //

        self.rate_enthalpy_change_vector.
            push(-heat_flowrate_from_cv_1_to_cv_2);
        single_cv_2.rate_enthalpy_change_vector.
            push(heat_flowrate_from_cv_1_to_cv_2);

        // relevant timescale here is courant number
        //
        // the timescale can only be calculated after the mass flows 
        // in and out of the cv are sufficiently calculated
        // the only thing we can do here is push the mass flowrate 
        // into the individual mass flowrate vectors 
        //
        // by convention, mass flowrate goes out of cv1 and into cv2 
        // so positive mass flowrate here is positive for cv2 
        // and negative for cv1
        //
        // I'll need a density for the flow first

        let density_cv1 = advection_data.fluid_density_heat_transfer_entity_1;
        let density_cv2 = advection_data.fluid_density_heat_transfer_entity_2;

        let volumetric_flowrate: VolumeRate;

        if mass_flow_from_cv_1_to_cv_2 > MassRate::zero() {
            // if mass flowrate is positive, flow is moving from cv1 
            // to cv2 
            // then the density we use is cv1 

            volumetric_flowrate = mass_flow_from_cv_1_to_cv_2/density_cv1;

        } else {
            // if mass flowrate is positive, flow is moving from cv2
            // to cv1
            // then the density we use is cv2

            volumetric_flowrate = mass_flow_from_cv_1_to_cv_2/density_cv2;
        }

        // now that I've done the volume flowrate calculation, push the 
        // volumetric flowrate to each vector
        self.volumetric_flowrate_vector.push(
            -volumetric_flowrate);
        single_cv_2.volumetric_flowrate_vector.push(
            volumetric_flowrate);



        // done! 
        Ok(())
    }

    /// calculates a suitable timescale when two single cv nodes interact
    pub fn calculate_mesh_stability_timestep_for_two_single_cv_nodes(
        &mut self,
        single_cv_2: &mut SingleCVNode,
        interaction: HeatTransferInteractionType) 
        -> Result<Time, ThermalHydraulicsLibError> 
    {

        let temperature_1: ThermodynamicTemperature = 
            self.temperature;

        let temperature_2: ThermodynamicTemperature = 
            single_cv_2.temperature;

        let pressure_1: Pressure = 
            self.pressure_control_volume;

        let pressure_2: Pressure = 
            single_cv_2.pressure_control_volume;

        let material_1 = self.material_control_volume;
        let material_2 = single_cv_2.material_control_volume;

        // we use this to get the diffusion coefficient for both control 
        // volumes

        let alpha_1: DiffusionCoefficient = 
            try_get_alpha_thermal_diffusivity(material_1,
                temperature_1,
                pressure_1)?;

        let alpha_2: DiffusionCoefficient = 
            try_get_alpha_thermal_diffusivity(material_2,
                temperature_2,
                pressure_2)?;

        // we also want to get the ratios of the two thermal inertias 

        let cp_1: SpecificHeatCapacity = try_get_cp(
            material_1,
            temperature_1,
            pressure_1)?;
        let cp_2: SpecificHeatCapacity = try_get_cp(
            material_2,
            temperature_2,
            pressure_2)?;

        let mass_1: Mass = self.mass_control_volume;
        let mass_2: Mass = single_cv_2.mass_control_volume;

        // we get heat capacity 

        let mcp_1: HeatCapacity = mass_1 * cp_1;
        let mcp_2: HeatCapacity = mass_2 * cp_2;

        // and the ratios of the heat capacities: 

        let heat_capacity_ratio_cv_1 = mcp_1 / (mcp_1 + mcp_2);
        let heat_capacity_ratio_cv_1: f64 = heat_capacity_ratio_cv_1.value;

        let heat_capacity_ratio_cv_2 = mcp_1 / (mcp_1 + mcp_2);
        let heat_capacity_ratio_cv_2: f64 = heat_capacity_ratio_cv_2.value;

        // the heat capacity ratios will be used to split up the length 
        // scales as needed

        // the strategy here is to find the shorter of the two lengthscales 
        // calculate the time step for each control volume, 
        // and then find the shortest timestep, then load both control volumes 
        // with the shortest time step
        let max_mesh_fourier_number: f64 = 0.25;

        // we'll calculate two baseline time scales
        let mut cv_1_timestep:Time = 
            self.calculate_conduction_timestep()?;

        let mut cv_2_timestep:Time = 
            single_cv_2.calculate_conduction_timestep()?;

        // this next step estimates the minimum timestep based 
        // on the interaction type, and then loads both control volumes 
        // with this minimum timestep

        match interaction {
            HeatTransferInteractionType::UserSpecifiedThermalConductance(user_specified_conductance) => {

                let lengthscale_stability_vec_1 = 
                    self.mesh_stability_lengthscale_vector.clone();
                let lengthscale_stability_vec_2 = 
                    single_cv_2.mesh_stability_lengthscale_vector.clone();

                let mut min_lengthscale = Length::new::<meter>(0.0);

                for length in lengthscale_stability_vec_1.iter() {
                    if *length > min_lengthscale {
                        min_lengthscale = *length;
                    }
                }

                for length in lengthscale_stability_vec_2.iter() {
                    if *length > min_lengthscale {
                        min_lengthscale = *length;
                    }
                }

                // now that we've gotten both length scales and gotten 
                // the shortest one, we'll need to obtain a timescale 
                // from the conductance 
                //
                // Delta t = Fo * rho * cp * Delta x * resistance
                //
                // we'll convert conductance into total resistance first 
                // 

                let total_resistance = 1.0/user_specified_conductance;

                // then we'll approximate the thermal resistance of 
                // cv 1 using the ratio of heat capacities 
                //
                // This is approximation, not exact, I'll deal with 
                // the approximation as I need to later.

                let approx_thermal_resistance_1= total_resistance *
                    heat_capacity_ratio_cv_1;

                let approx_thermal_cond_1: ThermalConductance = 
                    1.0/approx_thermal_resistance_1;

                // do note that thermal conductance is in watts per kelvin 
                // not watts per m2/K
                // area is factored in
                //
                // qA = H (T_1 - T_2)
                //
                // if i want HbyA or HbyV, then divide by the control volume
                //
                // H = kA/L for thermal conductivity case
                //  
                // bummer, no surface area!
                // I suppose I'll just use the minimum length scales 
                // then for conservative-ness

                let approx_thermal_conductivity_1: ThermalConductivity = 
                    approx_thermal_cond_1 / min_lengthscale;



                let approx_thermal_resistance_2 = total_resistance *
                    heat_capacity_ratio_cv_2;

                let approx_thermal_cond_2: ThermalConductance = 
                    1.0/approx_thermal_resistance_2;

                let approx_thermal_conductivity_2: ThermalConductivity = 
                    approx_thermal_cond_2 / min_lengthscale;

                // let's get timescale 1 and 2 estimate 

                let density_1: MassDensity 
                    = try_get_rho(material_1, temperature_1, pressure_1)?;

                let density_2: MassDensity 
                    = try_get_rho(material_2, temperature_2, pressure_2)?;

                let rho_cp_1: VolumetricHeatCapacity = density_1 * cp_1;
                let rho_cp_2: VolumetricHeatCapacity = density_2 * cp_2;

                let approx_alpha_1: DiffusionCoefficient = 
                    approx_thermal_conductivity_1/rho_cp_1;

                let approx_alpha_2: DiffusionCoefficient = 
                    approx_thermal_conductivity_2/rho_cp_2;


                // Fo  = alpha * Delta t / Delta x / Delta x 
                //
                // Delta t = Fo * Delta x * Delta x / alpha 
                //

                let timescale_1: Time = max_mesh_fourier_number 
                    * min_lengthscale 
                    * min_lengthscale 
                    / approx_alpha_1;

                let timescale_2: Time = max_mesh_fourier_number 
                    * min_lengthscale 
                    * min_lengthscale 
                    / approx_alpha_2;

                let interaction_minimum_timescale: Time; 

                if timescale_1 > timescale_2 {
                    interaction_minimum_timescale = timescale_2;
                } else {
                    interaction_minimum_timescale = timescale_1;
                }


                // take minimum of the timescales and assign it to 
                // the cv 1 or cv 2 timescales
                //

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }


            },
            HeatTransferInteractionType ::SingleCartesianThermalConductanceOneDimension(_material,thickness) => {

                // let's get the thickness and split it among cv 1 
                // and cv 2 

                let conduction_thickness: Length = thickness.into();

                let thickness_cv_1: Length = 
                    heat_capacity_ratio_cv_1 * 
                    conduction_thickness;

                let thickness_cv_2: Length = 
                    heat_capacity_ratio_cv_2 * 
                    conduction_thickness;

                // Fo  = alpha * Delta t / Delta x / Delta x 
                //
                // Delta t = Fo * Delta x * Delta x / alpha 
                //

                let timescale_1: Time = max_mesh_fourier_number 
                    * thickness_cv_1 
                    * thickness_cv_1 
                    / alpha_1;

                let timescale_2: Time = max_mesh_fourier_number 
                    * thickness_cv_2 
                    * thickness_cv_2 
                    / alpha_2;


                let interaction_minimum_timescale: Time; 

                if timescale_1 > timescale_2 {
                    interaction_minimum_timescale = timescale_2;
                } else {
                    interaction_minimum_timescale = timescale_1;
                }


                // take minimum of the timescales and assign it to 
                // the cv 1 or cv 2 timescales
                //

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }


            },
            HeatTransferInteractionType:: CylindricalConductionConvectionLiquidInside(
                (_solid_material, shell_thickness,
                 _solid_temperature, _solid_pressure), 
                (h, inner_diameter, _cylinder_length)
            ) => {

                // now for this, we are going to have to factor in 
                // h to get our nusselt number
                // unfortunately, we don't have a value for liquid 
                // thermal conductivity
                // not unless we match try matching the liquid 
                // material of either cv first 
                //
                // I'll just set it to material 1 by default

                let mut liquid_thermal_conductivity: ThermalConductivity
                    = try_get_kappa_thermal_conductivity(
                        material_1,
                        temperature_1,
                        pressure_1)?;

                // we assume liquid is material 1, solid is material 
                // 2 
                // if we are wrong, swop it around
                let mut alpha_liquid: DiffusionCoefficient = 
                    try_get_alpha_thermal_diffusivity(
                        material_1,
                        temperature_1,
                        pressure_1)?;

                let mut alpha_solid: DiffusionCoefficient = 
                    try_get_alpha_thermal_diffusivity(
                        material_2,
                        temperature_2,
                        pressure_2)?;

                match material_1 {
                    Material::Solid(_) => 
                    {
                        // if material 1 is solid, then we should obtain 
                        // liquid thermal conductivity from material 2
                        liquid_thermal_conductivity = try_get_kappa_thermal_conductivity(
                            material_2,
                            temperature_2,
                            pressure_2
                        )?;

                        alpha_solid = try_get_alpha_thermal_diffusivity(
                            material_1,
                            temperature_1,
                            pressure_1)?;

                        alpha_liquid = try_get_alpha_thermal_diffusivity(
                            material_2,
                            temperature_2,
                            pressure_2)?;
                        ()
                    },
                    Material::Liquid(_) => 
                    {
                        // if it's liquid material, match it and obtain 
                        // thermal conductivity 
                        // if the liquid is in material 1, do nothing
                        ()
                    },
                }



                // if both material 1 and 2 are solid at the same time 
                // or liquid at the same time, this is WRONG
                //
                // it's a runtime error, not quite ideal
                // obviously, want a compile time error
                // but wait till future to debug this

                match (material_1, material_2) {
                    (Material::Solid(_), Material::Solid(_)) => {
                        return Err(ThermalHydraulicsLibError::GenericStringError(
                                "should have 1 fluid and 1 solid".to_string()));
                    },
                    (Material::Liquid(_), Material::Liquid(_)) => {
                        return Err(ThermalHydraulicsLibError::GenericStringError(
                                "should have 1 fluid and 1 solid".to_string()));
                    },
                    _ => (),
                }

                // todo: probably need to check which lengthscale is 
                // right for nusselt number 

                let id: Length = inner_diameter.clone().into();
                let nusselt_number_diameter: Ratio = 
                    h * id / liquid_thermal_conductivity;

                let nusselt_number_diameter: f64 = 
                    nusselt_number_diameter.value;

                // normally time scale is 
                // Delta t = Fo * Delta x * Delta x / alpha 
                // 
                // if convection at boundary is taken into account:
                // Delta t = Fo * Delta x * Delta x / alpha Nu_(delta x)
                //
                //
                // for cylindrical control vol on inside, 
                // let the lengthscale be 
                // the radius

                let liquid_lengthscale: Length = id/2.0;

                let nusselt_number_radius = nusselt_number_diameter/2.0;

                let liquid_timescale: Time = max_mesh_fourier_number * 
                    liquid_lengthscale * 
                    liquid_lengthscale / 
                    alpha_liquid / 
                    nusselt_number_radius;

                // the solid lengthscale shall be half the thickness 
                // of the wall tubing 

                let solid_lengthscale: Length = shell_thickness.into();


                let solid_timescale: Time = max_mesh_fourier_number * 
                    solid_lengthscale * 
                    solid_lengthscale / 
                    alpha_solid ;

                // we find the minimum timescale and append it 
                // to both control volumes

                let interaction_minimum_timescale: Time; 

                if solid_timescale > liquid_timescale {
                    interaction_minimum_timescale = liquid_timescale;
                } else {
                    interaction_minimum_timescale = solid_timescale;
                }


                // take minimum of the timescales and assign it to 
                // the cv 1 or cv 2 timescales
                //

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }

            },

            HeatTransferInteractionType:: CylindricalConductionConvectionLiquidOutside(
                (_solid_material, shell_thickness,
                 _solid_temperature, _solid_pressure), 
                (h, _outer_diameter, _cylinder_length)
            ) => {
                // 
                //
                // now for liquid on the outside, we really have no idea 
                // how much thermal inertia it has based on the 
                // lengthscale, so we don't have 
                // a clue as to its length scale directly
                // 
                // however, we can guestimate the lengthscale of the fluid 
                // volume by taking the lengthscale of the solid, and then 
                // scaling it by the ratio of the thermal inertia of 
                // the solid to the liquid 

                let thickness: Length = shell_thickness.clone().into();

                let solid_lengthscale = thickness;

                // to get liquid lengthscale, we must first find out 
                // which control volume contains the liquid 

                //
                // I'll just set it to material 1 by default

                let mut liquid_thermal_conductivity: ThermalConductivity
                    = try_get_kappa_thermal_conductivity(
                        material_1,
                        temperature_1,
                        pressure_1)?;

                let mut liquid_mcp = mcp_1;
                let mut solid_mcp = mcp_2;

                // we assume liquid is material 1, solid is material 
                // 2 
                // if we are wrong, swop it around
                let mut alpha_liquid: DiffusionCoefficient = 
                    try_get_alpha_thermal_diffusivity(
                        material_1,
                        temperature_1,
                        pressure_1)?;

                let mut alpha_solid: DiffusionCoefficient = 
                    try_get_alpha_thermal_diffusivity(
                        material_2,
                        temperature_2,
                        pressure_2)?;

                match material_1 {
                    Material::Solid(_) => {
                        // if material 1 is solid, then we should obtain 
                        // liquid thermal conductivity from material 2
                        liquid_thermal_conductivity = try_get_kappa_thermal_conductivity(
                            material_2,
                            temperature_2,
                            pressure_2
                        )?;

                        alpha_solid = try_get_alpha_thermal_diffusivity(
                            material_1,
                            temperature_1,
                            pressure_1)?;

                        alpha_liquid = try_get_alpha_thermal_diffusivity(
                            material_2,
                            temperature_2,
                            pressure_2)?;

                        liquid_mcp = mcp_2;
                        solid_mcp = mcp_1;
                        ()
                    },
                    Material::Liquid(_) => {
                        // if it's liquid material, match it and obtain 
                        // thermal conductivity 
                        // if the liquid is in material 1, do nothing
                        ()
                    },
                }

                // probably should be taken as cube root but maybe later 
                let liquid_lengthscale: Length = solid_lengthscale * 
                    liquid_mcp / 
                    solid_mcp;


                let solid_timescale: Time = max_mesh_fourier_number * 
                    solid_lengthscale * 
                    solid_lengthscale / 
                    alpha_solid ;


                // Now, I'm going to use the liquid lengthscale 
                // rather than the radius as the lengthscale is a measure 
                // of the thermal inertia 
                //
                // whether it's linear or cube root, I'm not too sure now 
                // didn't bother checking or otherwise. 
                //
                // I'm pretty sure it's cube rooted
                // but I'll correct that later if we get issues

                let nusselt_number: Ratio = 
                    h * liquid_lengthscale / liquid_thermal_conductivity;

                let liquid_timescale: Time = max_mesh_fourier_number * 
                    liquid_lengthscale * 
                    liquid_lengthscale / 
                    alpha_liquid / 
                    nusselt_number;

                // now set the control volume using the minimum timescales

                let interaction_minimum_timescale: Time; 

                if solid_timescale > liquid_timescale {
                    interaction_minimum_timescale = liquid_timescale;
                } else {
                    interaction_minimum_timescale = solid_timescale;
                }


                // take minimum of the timescales and assign it to 
                // the cv 1 or cv 2 timescales
                //

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }

            },
            // note: actually function signatures are a little more 
            // friendly to use than packing enums with lots of stuff 
            // so may change stuffing enums with tuples to stuffing 
            // enums with a single struct
            HeatTransferInteractionType ::DualCylindricalThermalConductance(
                (inner_material,inner_shell_thickness),
                (outer_material,outer_shell_thickness),
                (inner_diameter,
                 outer_diameter,
                 _cylinder_length)) => 
            {
                // first, want to check if inner_diameter + 
                // shell thicknesses is outer diameter 

                let expected_outer_diameter: Length;
                let id: Length = inner_diameter.into();
                let inner_thickness: Length =  inner_shell_thickness.into();
                let outer_thickness: Length =  outer_shell_thickness.into();

                expected_outer_diameter = 
                    id + inner_thickness + outer_thickness;

                let od: Length = outer_diameter.into();

                // inner diameter and outer diameter values must be 
                // equal to within 1 nanometer 1e-9 m
                if (od.value - expected_outer_diameter.value).abs() > 1e-9
                {

                    let mut error_str: String = "the inner diameter  \n
                                            plus shell thicknesses do not equate  \n
                                            to outer diameter".to_string();

                    error_str += "supplied outer diameter (m):";
                    error_str += &od.value.to_string();
                    error_str += "expected outer diameter (m):";
                    error_str += &expected_outer_diameter.value.to_string();


                    return Err(ThermalHydraulicsLibError::GenericStringError(error_str));
                }

                // for this, it should be quite straight forward, 
                // find both alphas 
                //
                // by convention, material 1 is inner shell 
                // and material 2 is outer shell 

                let inner_shell_material = inner_material; 
                let inner_shell_temperature = temperature_1;
                let inner_shell_pressure = pressure_1;

                let outer_shell_material = outer_material; 
                let outer_shell_temperature = temperature_2;
                let outer_shell_pressure = pressure_2;


                let alpha_inner: DiffusionCoefficient = 
                    try_get_alpha_thermal_diffusivity(inner_shell_material,
                        inner_shell_temperature,
                        inner_shell_pressure)?;

                let alpha_outer: DiffusionCoefficient = 
                    try_get_alpha_thermal_diffusivity(outer_shell_material,
                        outer_shell_temperature,
                        outer_shell_pressure)?;

                // now let's calculate timescales of both shells 
                // this pertains to conduction only, no convection

                let inner_shell_timescale: Time = max_mesh_fourier_number * 
                    inner_thickness * 
                    inner_thickness / 
                    alpha_inner;

                let outer_shell_timescale: Time = max_mesh_fourier_number * 
                    outer_thickness * 
                    outer_thickness / 
                    alpha_outer;


                // now adjust timestep again

                let interaction_minimum_timescale: Time; 

                if outer_shell_timescale > inner_shell_timescale {
                    interaction_minimum_timescale = inner_shell_timescale;
                } else {
                    interaction_minimum_timescale = outer_shell_timescale;
                }

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }


            },

            HeatTransferInteractionType::UserSpecifiedHeatAddition => 
            {
                // user specified heat addition does not make sense 
                // for cv to cv interaction
                println!("interaction type needs to be thermal conductance");
                return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
            },

            HeatTransferInteractionType:: UserSpecifiedHeatFluxCustomArea(_) => 
            {

                // user specified heat flux does not make sense 
                // for cv to cv interaction
                println!("interaction type needs to be thermal conductance");

                return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);

            },
            HeatTransferInteractionType:: UserSpecifiedHeatFluxCylindricalOuterArea(_,_) => 
            {

                // user specified heat flux does not make sense 
                // for cv to cv interaction
                println!("interaction type needs to be thermal conductance");

                return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
            },
            HeatTransferInteractionType:: UserSpecifiedHeatFluxCylindricalInnerArea(_,_) => 
            {

                // user specified heat flux does not make sense 
                // for cv to cv interaction
                println!("interaction type needs to be thermal conductance");

                return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
            },
            HeatTransferInteractionType:: DualCartesianThermalConductance(
                (_material_1, thickness_1),
                (_material_2,thickness_2)
            ) => { 

                let thickness_1_length: Length = thickness_1.into();
                let thickness_2_length: Length = thickness_2.into();


                // now, we are not quite sure which material 
                // corresponds to which control volume,
                // though of course, we know both are solids 
                // 
                // and that 
                // Delta t = Fo * Delta x * Delta x  / alpha 
                //
                // so to get the minimum delta t, we take the 
                // shorter of the two lengths, and the larger of 
                // the two alphas 
                //

                let mut larger_alpha: DiffusionCoefficient = 
                    alpha_1;

                if alpha_1 < alpha_2 {
                    larger_alpha = alpha_2;
                }


                let mut smaller_lengthscale: Length = thickness_1.into();

                if thickness_2_length < thickness_1_length {
                    smaller_lengthscale = thickness_2_length;
                }

                let interaction_minimum_timescale: Time 
                    = max_mesh_fourier_number * 
                    smaller_lengthscale *
                    smaller_lengthscale / 
                    larger_alpha;

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }

            },
            HeatTransferInteractionType::DualCartesianThermalConductanceThreeDimension(
                data_dual_cartesian_conduction
            ) => {


                let thickness_1 = 
                    data_dual_cartesian_conduction .thickness_1;

                let thickness_2 = 
                    data_dual_cartesian_conduction .thickness_2;


                let thickness_1_length: Length = thickness_1.into();
                let thickness_2_length: Length = thickness_2.into();


                // now, we are not quite sure which material 
                // corresponds to which control volume,
                // though of course, we know both are solids 
                // 
                // and that 
                // Delta t = Fo * Delta x * Delta x  / alpha 
                //
                // so to get the minimum delta t, we take the 
                // shorter of the two lengths, and the larger of 
                // the two alphas 
                //

                let mut larger_alpha: DiffusionCoefficient = 
                    alpha_1;

                if alpha_1 < alpha_2 {
                    larger_alpha = alpha_2;
                }


                let mut smaller_lengthscale: Length = thickness_1.into();

                if thickness_2_length < thickness_1_length {
                    smaller_lengthscale = thickness_2_length;
                }

                let interaction_minimum_timescale: Time 
                    = max_mesh_fourier_number * 
                    smaller_lengthscale *
                    smaller_lengthscale / 
                    larger_alpha;

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }

            },

            HeatTransferInteractionType:: UserSpecifiedConvectionResistance(
                data_convection_resistance
            ) => {

                // repeat analysis as per user specified thermal 
                // conductance

                let heat_transfer_coeff: HeatTransfer = 
                    data_convection_resistance.heat_transfer_coeff;
                let surf_area: Area = 
                    data_convection_resistance.surf_area.into();

                let user_specified_conductance: ThermalConductance = 
                    heat_transfer_coeff * surf_area;

                let lengthscale_stability_vec_1 = 
                    self.mesh_stability_lengthscale_vector.clone();
                let lengthscale_stability_vec_2 = 
                    single_cv_2.mesh_stability_lengthscale_vector.clone();

                let mut min_lengthscale = Length::new::<meter>(0.0);

                for length in lengthscale_stability_vec_1.iter() {
                    if *length > min_lengthscale {
                        min_lengthscale = *length;
                    }
                }

                for length in lengthscale_stability_vec_2.iter() {
                    if *length > min_lengthscale {
                        min_lengthscale = *length;
                    }
                }

                // now that we've gotten both length scales and gotten 
                // the shortest one, we'll need to obtain a timescale 
                // from the conductance 
                //
                // Delta t = Fo * rho * cp * Delta x * resistance
                //
                // we'll convert conductance into total resistance first 
                // 

                let total_resistance = 1.0/user_specified_conductance;

                // then we'll approximate the thermal resistance of 
                // cv 1 using the ratio of heat capacities 
                //
                // This is approximation, not exact, I'll deal with 
                // the approximation as I need to later.

                let approx_thermal_resistance_1= total_resistance *
                    heat_capacity_ratio_cv_1;

                let approx_thermal_cond_1: ThermalConductance = 
                    1.0/approx_thermal_resistance_1;

                // do note that thermal conductance is in watts per kelvin 
                // not watts per m2/K
                // area is factored in
                //
                // qA = H (T_1 - T_2)
                //
                // if i want HbyA or HbyV, then divide by the control volume
                //
                // H = kA/L for thermal conductivity case
                //  
                // bummer, no surface area!
                // I suppose I'll just use the minimum length scales 
                // then for conservative-ness

                let approx_thermal_conductivity_1: ThermalConductivity = 
                    approx_thermal_cond_1 / min_lengthscale;



                let approx_thermal_resistance_2 = total_resistance *
                    heat_capacity_ratio_cv_2;

                let approx_thermal_cond_2: ThermalConductance = 
                    1.0/approx_thermal_resistance_2;

                let approx_thermal_conductivity_2: ThermalConductivity = 
                    approx_thermal_cond_2 / min_lengthscale;

                // let's get timescale 1 and 2 estimate 

                let density_1: MassDensity 
                    = try_get_rho(material_1, temperature_1, pressure_1)?;

                let density_2: MassDensity 
                    = try_get_rho(material_2, temperature_2, pressure_2)?;

                let rho_cp_1: VolumetricHeatCapacity = density_1 * cp_1;
                let rho_cp_2: VolumetricHeatCapacity = density_2 * cp_2;

                let approx_alpha_1: DiffusionCoefficient = 
                    approx_thermal_conductivity_1/rho_cp_1;

                let approx_alpha_2: DiffusionCoefficient = 
                    approx_thermal_conductivity_2/rho_cp_2;


                // Fo  = alpha * Delta t / Delta x / Delta x 
                //
                // Delta t = Fo * Delta x * Delta x / alpha 
                //

                let timescale_1: Time = max_mesh_fourier_number 
                    * min_lengthscale 
                    * min_lengthscale 
                    / approx_alpha_1;

                let timescale_2: Time = max_mesh_fourier_number 
                    * min_lengthscale 
                    * min_lengthscale 
                    / approx_alpha_2;

                let interaction_minimum_timescale: Time; 

                if timescale_1 > timescale_2 {
                    interaction_minimum_timescale = timescale_2;
                } else {
                    interaction_minimum_timescale = timescale_1;
                }


                // take minimum of the timescales and assign it to 
                // the cv 1 or cv 2 timescales
                //

                if cv_1_timestep > interaction_minimum_timescale {
                    cv_1_timestep = interaction_minimum_timescale;
                }

                if cv_2_timestep > interaction_minimum_timescale {
                    cv_2_timestep = interaction_minimum_timescale;
                }


            },
            HeatTransferInteractionType::Advection(_) => 
            {
                // advection has nothing to do with mesh stability timestep 
                // do nothing
                //
                // it can only be calculated after the total mass flowrates 
                // in and out of the control volumes are calculated

                ()
            },

            HeatTransferInteractionType::
                SimpleRadiation
                (_area_coeff, _hot_temperature, _cold_temperature) => 
                {
                    // while radiation can be treated as conduction 
                    // in optically thick media, i'm not implementing 
                    // this yet
                    ()
                }
            ,
        };

        // push the corrected minimum timesteps to cv 1 and cv 2
        self.max_timestep_vector.push(cv_1_timestep);
        single_cv_2.max_timestep_vector.push(cv_2_timestep);

        // load the minimum timestep and return
        let mut minimum_timestep: Time = cv_1_timestep;

        if cv_1_timestep > cv_2_timestep {
            minimum_timestep = cv_2_timestep;
        }

        return Ok(minimum_timestep);
    }
}

