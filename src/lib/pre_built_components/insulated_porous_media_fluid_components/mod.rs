use std::f64::consts::PI;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::*;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::NusseltPrandtlReynoldsData;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::WakaoData;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;

use super::heat_transfer_entities::HeatTransferEntity;
use uom::si::area::square_inch;
use uom::si::f64::*;
use uom::si::area::square_meter;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::inch;
use uom::si::length::meter;
use uom::si::ratio::ratio;
use uom::si::pressure::atmosphere;
use uom::ConstZero;
/// Fluid Components with Internals 
/// 
/// This could be an insulated pipe with some twisted tape inside 
/// For example, a static mixer
/// 
/// StaticMixer MX-10 is a classic example of what this class is meant for 
/// 
/// However, it could also be used for CIET Heater v1.0 where it was insulated
/// and had an annular pipe inside it
/// 
#[derive(Debug,Clone,PartialEq)]
pub struct InsulatedPorousMediaFluidComponent {

    inner_nodes: usize,

    /// heat transfer entity representing control volumes 
    /// for the insulation around the Insulated Porous media component 
    /// such as MX-10
    pub insulation_array: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// of heat generating or 
    /// non-heat generating components within the pipe 
    /// or fluid component 
    ///
    /// for example,
    /// the twisted tape in the heated section of CIET's Heater
    pub interior_solid_array_for_porous_media: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the steel piping in MX-10
    pub pipe_shell: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the therminol fluid in MX-10
    pub pipe_fluid_array: HeatTransferEntity,

    /// ambient temperature of air used to calculate heat loss
    pub ambient_temperature: ThermodynamicTemperature,

    /// heat transfer coefficient used to calculate heat loss 
    /// to air
    pub heat_transfer_to_ambient: HeatTransfer,


    flow_area: Area,

    /// loss correlations
    /// for pipe losses
    pub darcy_loss_correlation: DimensionlessDarcyLossCorrelations,

    /// thermal conductance lengthscale to ambient 
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length,

    /// thermal conductance lengthscale from pipe to fluid
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub thermal_conductance_lengthscale_pipe_shell_to_fluid: Length,

    /// thermal conductance lengthscale from fluid to 
    /// porous media internal
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length,

    /// thermal conductance lengthscale from pipe shell to insulation
    /// 
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length,

    /// thermal conductance lengthscale from pipe shell to insulation
    /// 
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub thermal_conductance_lengthscale_insulation_to_ambient: Length,

    /// nusselt correlation from fluid to pipe shell 
    pub nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation,

    /// lengthscale for nusselt correlation to ambient 
    /// for pipes, the hydraulic diameter usually suffices 
    pub nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length,

    /// convection heat transfer area to ambient 
    /// used to calculate conductance to ambient hA
    /// conductance = h A 
    pub convection_heat_transfer_area_insulation_to_ambient: Area,

    /// nusselt correlation to porous media interior
    pub nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation,

    /// lengthscale for nusselt correlation to porous_media_interior 
    /// for pipes, the hydraulic diameter usually suffices 
    pub nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length,

    /// convection heat transfer area to pipe 
    /// used to calculate conductance to pipe hA
    /// conductance = h A 
    pub convection_heat_transfer_area_fluid_to_pipe_shell: Area,

    /// convection heat transfer area to interior 
    /// used to calculate conductance to interior hA
    /// conductance = h A 
    pub convection_heat_transfer_area_fluid_to_interior: Area,
}

impl InsulatedPorousMediaFluidComponent {


    /// traditional callibrated heater constructor 
    /// with 20 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    /// However, there is a layer of insulation over it
    /// insulation thickness is estimated at 5.08 cm according 
    /// to Zweibaum's RELAP model.
    ///
    /// This is mainly to ensure that the InsulatedPorousMediaFluidComponent 
    /// is programmed correctly,
    /// because it will be checked against exactly the same 
    /// component without the insulation
    pub fn new_dewet_model_heater_v2_insulated(
        initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(0.01467);
        let insulation_thicnkess = Length::new::<meter>(0.0508);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
        let steel_shell_id = Length::new::<meter>(0.0381);
        let steel_shell_od = Length::new::<meter>(0.04);

        let insulation_id = steel_shell_od;
        let insulation_od = insulation_id + 2.0 * insulation_thicnkess;


        // inner therminol array 
        //
        // the darcy loss correlation is f = 17.9 *Re^{-0.34}
        // accurate to within 4% (Lukas et al)
        // Improved Heat Transfer and Volume Scaling through 
        // Novel Heater Design
        // 

        let a = Ratio::ZERO;
        let b = Ratio::new::<ratio>(17.9);
        let c: f64  = -0.34;
        let mut therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            a,
            b,
            c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );

        // the therminol array nusselt correlation should be that of the 
        // heater 

        let heater_prandtl_reynolds_data: NusseltPrandtlReynoldsData 
        = NusseltPrandtlReynoldsData::default();
        therminol_array.nusselt_correlation = 
            NusseltCorrelation::CIETHeaterVersion2(
                heater_prandtl_reynolds_data
                );

        let darcy_loss_correlation = 
            therminol_array.fluid_component_loss_properties.clone();
        // the therminol arrays here use gnielinski correlation by 
        // default

        let wakao_correlation = NusseltCorrelation::Wakao(
            WakaoData::default()
        );

        

        // now, nusselt correlation to ambient and to porous media 
        // are the same, I did not do anything special because 
        // transient validation was not important (yet) 
        // when I originally wrote this code 
        let nusselt_correlation_fluid_to_pipe_shell = therminol_array.nusselt_correlation;
        let nusselt_correlation_fluid_to_porous_media_interior = wakao_correlation;
        let nusselt_correlation_lengthscale_fluid_to_pipe_shell = hydraulic_diameter;
        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior = hydraulic_diameter;

        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            steel_shell_id,
            steel_shell_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        // for thermal conductance lengthscale for cylinder, we 
        // the easiest way is to get the actual conductance 
        // which is in terms of (kA/L) then divide by the conductivity
        let steel_shell_mid_diameter: Length = (steel_shell_od + steel_shell_id)/2.0;
        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();
        
        let steel_shell_conductance_to_ambient: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_shell_od,
                heated_length,
                steel_thermal_conductivity).unwrap();

        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_id,
                steel_shell_mid_diameter,
                heated_length,
                steel_thermal_conductivity).unwrap();


        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length = 
            steel_shell_conductance_to_ambient/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length = 
            steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        // for this iteration of the heater, I'm kind of lazy 
        // my conductance lengthscale to the porous media interior
        // is kind of guesswork
        //
        // I could put a large number here to to neglect the 
        // resistance of the porous media fluid
        // In fact, when calculating thermal resistance for the 
        // twisted tape, I ignored the resistance
        // of the twisted tape, so just put a large number here
        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);
            

        // now twisted_tape 
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        // next is the insulation array 
        let insulation_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            insulation_id,
            insulation_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );
        let insulation_mid_diameter: Length = (insulation_od + insulation_id)/2.0;
        let insulation_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();
        
        let insulation_conductance_to_ambient: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                insulation_mid_diameter,
                insulation_od,
                heated_length,
                insulation_thermal_conductivity).unwrap();

        let insulation_conductance_to_pipe_insulation_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                insulation_id,
                insulation_mid_diameter,
                heated_length,
                insulation_thermal_conductivity).unwrap();


        let thermal_conductance_lengthscale_insulation_to_ambient: Length = 
            insulation_conductance_to_ambient/insulation_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length = 
            insulation_conductance_to_pipe_insulation_boundary/insulation_thermal_conductivity;


        // convection heat transfer area to interior (twisted tape)
        // this is approximate btw
        //
        // I just copied this straight from the preprocessing 
        // bit to be consistent

        // find suitable heat transfer area
        let heated_length = Length::new::<meter>(1.6383);
        let heated_length_plus_heads = Length::new::<inch>(78.0);

        let heat_transfer_area_heated_length_plus_heads: Area = 
            Area::new::<square_inch>(719.0);

        let heat_transfer_area_heated_length_only: Area
            = heated_length/ heated_length_plus_heads * 
            heat_transfer_area_heated_length_plus_heads;

        let convection_heat_transfer_area_fluid_to_interior = 
            heat_transfer_area_heated_length_only;
        // area = PI * inner diameter * L
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area 
            = PI * steel_shell_id * heated_length;



        // area = PI * outer diameter * L 
        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * insulation_od * heated_length;

        return Self { inner_nodes: user_specified_inner_nodes,
            interior_solid_array_for_porous_media: twisted_tape.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            insulation_array: insulation_array.into(),
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            convection_heat_transfer_area_insulation_to_ambient,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            nusselt_correlation_fluid_to_porous_media_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,

        };
    }

    /// constructs the static mixer using the RELAP/SAM model 
    /// as a basis 
    ///
    /// static mixer 20 (MX-20) on CIET diagram
    /// in the DRACS branch in primary loop
    /// just after the DRACS heat exchanger
    /// from top to bottom
    /// label 23
    ///
    /// in reality flow goes from bottom to
    /// top in natural convection
    /// also in the DRACS
    /// loop there are flow diodes to make
    /// it such that flow going from bottom to top
    /// encounters more resistance
    ///
    /// original angle is is 90 degrees
    /// but i orientate from top to bottom
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    /// Reynolds Number Correlation: 21 + 4000/Re
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_23_mx20(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.33);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let correlation_constant_a = Ratio::new::<ratio>(21.0);
        let correlation_coeff_b = Ratio::new::<ratio>(4000.0);
        let reynolds_power_c: f64 = -1.0;


        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            correlation_constant_a,
            correlation_coeff_b,
            reynolds_power_c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // for the porous media internal I am using the twisted 
        // tape dimensions as an estimate
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = component_length;

        let porous_media_internal = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
            


        // for new code, I need a lot of lengthscales and area scales 

        // so first, areas, usuall PI * d * l
        //
        // note that we don't actually have heat transfer areas of 
        // the interior for static mixers, so I'm just using a guestimate
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area
            = PI * steel_id * component_length;

        let convection_heat_transfer_area_fluid_to_interior: Area
            = convection_heat_transfer_area_fluid_to_pipe_shell * 2.0;

        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * fiberglass_od * component_length;

        // now nusselt correlations
        // I'm also guestimating that it is for the inside,
        // there was no experimental data or whatsover.
        let nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation 
            = therminol_array.nusselt_correlation.clone();

        let nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation 
            = nusselt_correlation_fluid_to_pipe_shell.clone();

        // now for the nusselt correlation lengthscales

        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let fiberglass_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let steel_shell_mid_diameter: Length = (steel_od + steel_id)/2.0;

        let fiberglass_shell_mid_diameter: Length 
            = (fiberglass_od + fiberglass_id)/2.0;


        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_id,
                steel_shell_mid_diameter,
                component_length,
                steel_thermal_conductivity).unwrap();


        let steel_shell_conductance_to_insulation_pipe_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_od,
                component_length,
                steel_thermal_conductivity).unwrap();

        let insulation_conductance_to_insulation_pipe_boundary: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_id,
                fiberglass_shell_mid_diameter,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let insulation_conductance_to_ambient: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_shell_mid_diameter,
                fiberglass_od,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length 
            = hydraulic_diameter;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length 
            = steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length 
            = steel_shell_conductance_to_insulation_pipe_boundary/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length 
            = insulation_conductance_to_insulation_pipe_boundary/fiberglass_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_ambient: Length 
            = insulation_conductance_to_ambient/fiberglass_thermal_conductivity;

        // for thermal conductance lengthscale for inner array,
        // I am guestimating
        //
        // just ignoring the thermal resistance of the 
        // insides, like lumped capacitance essentially

        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);

        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length 
            = hydraulic_diameter;

        // f + L/D K = 21 + 4000/Re
        let darcy_loss_correlation = 
            DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                Ratio::new::<ratio>(21.0),
                Ratio::new::<ratio>(4000.0),
                -1.0
            );

        return Self { inner_nodes: user_specified_inner_nodes,
        insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            interior_solid_array_for_porous_media: porous_media_internal.into(),
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            convection_heat_transfer_area_insulation_to_ambient,
            nusselt_correlation_fluid_to_porous_media_interior,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,
        };
    }

    /// constructs the static mixer using the RELAP/SAM model 
    /// as a basis 
    ///
    /// static mixer 21 (MX-21) on CIET diagram
    /// in the DHX branch in primary loop
    /// just before the DRACS heat exchanger
    /// from top to bottom
    /// label 25
    ///
    /// in reality flow goes from bottom to
    /// top in natural convection
    /// also in the DRACS
    /// loop there are flow diodes to make
    /// it such that flow going from bottom to top
    /// encounters more resistance
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    /// Reynolds Number Correlation: 21 + 4000/Re
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_25_mx21(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.33);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let correlation_constant_a = Ratio::new::<ratio>(21.0);
        let correlation_coeff_b = Ratio::new::<ratio>(4000.0);
        let reynolds_power_c: f64 = -1.0;



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            correlation_constant_a,
            correlation_coeff_b,
            reynolds_power_c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );
        // for the porous media internal I am using the twisted 
        // tape dimensions as an estimate
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = component_length;

        let porous_media_internal = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
            


        // for new code, I need a lot of lengthscales and area scales 

        // so first, areas, usuall PI * d * l
        //
        // note that we don't actually have heat transfer areas of 
        // the interior for static mixers, so I'm just using a guestimate
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area
            = PI * steel_id * component_length;

        let convection_heat_transfer_area_fluid_to_interior: Area
            = convection_heat_transfer_area_fluid_to_pipe_shell * 2.0;

        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * fiberglass_od * component_length;

        // now nusselt correlations
        // I'm also guestimating that it is for the inside,
        // there was no experimental data or whatsover.
        let nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation 
            = therminol_array.nusselt_correlation.clone();

        let nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation 
            = nusselt_correlation_fluid_to_pipe_shell.clone();

        // now for the nusselt correlation lengthscales

        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let fiberglass_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let steel_shell_mid_diameter: Length = (steel_od + steel_id)/2.0;

        let fiberglass_shell_mid_diameter: Length 
            = (fiberglass_od + fiberglass_id)/2.0;


        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_id,
                steel_shell_mid_diameter,
                component_length,
                steel_thermal_conductivity).unwrap();


        let steel_shell_conductance_to_insulation_pipe_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_od,
                component_length,
                steel_thermal_conductivity).unwrap();

        let insulation_conductance_to_insulation_pipe_boundary: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_id,
                fiberglass_shell_mid_diameter,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let insulation_conductance_to_ambient: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_shell_mid_diameter,
                fiberglass_od,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length 
            = hydraulic_diameter;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length 
            = steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length 
            = steel_shell_conductance_to_insulation_pipe_boundary/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length 
            = insulation_conductance_to_insulation_pipe_boundary/fiberglass_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_ambient: Length 
            = insulation_conductance_to_ambient/fiberglass_thermal_conductivity;

        // for thermal conductance lengthscale for inner array,
        // I am guestimating
        //
        // just ignoring the thermal resistance of the 
        // insides, like lumped capacitance essentially

        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);

        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length 
            = hydraulic_diameter;

        // f + L/D K = 21 + 4000/Re
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                Ratio::new::<ratio>(21.0),
                Ratio::new::<ratio>(4000.0),
                -1.0
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            interior_solid_array_for_porous_media: porous_media_internal.into(),
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            convection_heat_transfer_area_insulation_to_ambient,
            nusselt_correlation_fluid_to_porous_media_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
        };
    }

    /// constructs the static mixer using the RELAP/SAM model 
    /// as a basis 
    ///
    /// length = 0.33 m 
    /// d_h = 2.79e-2
    /// Insulation thickness: 5.08 cm
    /// (fiberglass)
    /// number of nodes (including two ends): 2
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    /// Reynolds Number Correlation: 21 + 4000/Re
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_2_mx10(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.33);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let correlation_constant_a = Ratio::new::<ratio>(21.0);
        let correlation_coeff_b = Ratio::new::<ratio>(4000.0);
        let reynolds_power_c: f64 = -1.0;



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            correlation_constant_a,
            correlation_coeff_b,
            reynolds_power_c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );
        // for the porous media internal I am using the twisted 
        // tape dimensions as an estimate
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = component_length;

        let porous_media_internal = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
            


        // for new code, I need a lot of lengthscales and area scales 

        // so first, areas, usuall PI * d * l
        //
        // note that we don't actually have heat transfer areas of 
        // the interior for static mixers, so I'm just using a guestimate
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area
            = PI * steel_id * component_length;

        let convection_heat_transfer_area_fluid_to_interior: Area
            = convection_heat_transfer_area_fluid_to_pipe_shell * 2.0;

        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * fiberglass_od * component_length;

        // now nusselt correlations
        // I'm also guestimating that it is for the inside,
        // there was no experimental data or whatsover.
        let nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation 
            = therminol_array.nusselt_correlation.clone();

        let nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation 
            = nusselt_correlation_fluid_to_pipe_shell.clone();

        // now for the nusselt correlation lengthscales

        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let fiberglass_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let steel_shell_mid_diameter: Length = (steel_od + steel_id)/2.0;

        let fiberglass_shell_mid_diameter: Length 
            = (fiberglass_od + fiberglass_id)/2.0;


        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_id,
                steel_shell_mid_diameter,
                component_length,
                steel_thermal_conductivity).unwrap();


        let steel_shell_conductance_to_insulation_pipe_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_od,
                component_length,
                steel_thermal_conductivity).unwrap();

        let insulation_conductance_to_insulation_pipe_boundary: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_id,
                fiberglass_shell_mid_diameter,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let insulation_conductance_to_ambient: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_shell_mid_diameter,
                fiberglass_od,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length 
            = hydraulic_diameter;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length 
            = steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length 
            = steel_shell_conductance_to_insulation_pipe_boundary/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length 
            = insulation_conductance_to_insulation_pipe_boundary/fiberglass_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_ambient: Length 
            = insulation_conductance_to_ambient/fiberglass_thermal_conductivity;

        // for thermal conductance lengthscale for inner array,
        // I am guestimating
        //
        // just ignoring the thermal resistance of the 
        // insides, like lumped capacitance essentially

        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);

        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length 
            = hydraulic_diameter;

        // f + L/D K = 21 + 4000/Re
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                Ratio::new::<ratio>(21.0),
                Ratio::new::<ratio>(4000.0),
                -1.0
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            interior_solid_array_for_porous_media: porous_media_internal.into(),
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            convection_heat_transfer_area_insulation_to_ambient,
            nusselt_correlation_fluid_to_porous_media_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
        };
    }

    /// constructs the static mixer pipe using the RELAP/SAM model 
    /// as a basis 
    ///
    /// length = 0.149425 m 
    /// d_h = 2.79e-2
    /// Insulation thickness: 5.08 cm
    /// (fiberglass)
    /// number of nodes (including two ends): 2
    ///
    /// form loss: 1.8
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_pipe_2a_mx10(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.149425);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let form_loss = Ratio::new::<ratio>(1.8);



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );
        // for the porous media internal I am using the twisted 
        // tape dimensions as an estimate
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = component_length;

        let porous_media_internal = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
            


        // for new code, I need a lot of lengthscales and area scales 

        // so first, areas, usuall PI * d * l
        //
        // note that we don't actually have heat transfer areas of 
        // the interior for static mixers, so I'm just using a guestimate
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area
            = PI * steel_id * component_length;

        let convection_heat_transfer_area_fluid_to_interior: Area
            = convection_heat_transfer_area_fluid_to_pipe_shell * 2.0;

        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * fiberglass_od * component_length;

        // now nusselt correlations
        // I'm also guestimating that it is for the inside,
        // there was no experimental data or whatsover.
        let nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation 
            = therminol_array.nusselt_correlation.clone();

        let nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation 
            = nusselt_correlation_fluid_to_pipe_shell.clone();

        // now for the nusselt correlation lengthscales

        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let fiberglass_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let steel_shell_mid_diameter: Length = (steel_od + steel_id)/2.0;

        let fiberglass_shell_mid_diameter: Length 
            = (fiberglass_od + fiberglass_id)/2.0;


        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_id,
                steel_shell_mid_diameter,
                component_length,
                steel_thermal_conductivity).unwrap();


        let steel_shell_conductance_to_insulation_pipe_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_od,
                component_length,
                steel_thermal_conductivity).unwrap();

        let insulation_conductance_to_insulation_pipe_boundary: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_id,
                fiberglass_shell_mid_diameter,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let insulation_conductance_to_ambient: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_shell_mid_diameter,
                fiberglass_od,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length 
            = hydraulic_diameter;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length 
            = steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length 
            = steel_shell_conductance_to_insulation_pipe_boundary/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length 
            = insulation_conductance_to_insulation_pipe_boundary/fiberglass_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_ambient: Length 
            = insulation_conductance_to_ambient/fiberglass_thermal_conductivity;

        // for thermal conductance lengthscale for inner array,
        // I am guestimating
        //
        // just ignoring the thermal resistance of the 
        // insides, like lumped capacitance essentially

        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);

        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length 
            = hydraulic_diameter;

        // K = 1.8 in a pipe
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_pipe(
                component_length,
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(),
                hydraulic_diameter,
                form_loss
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            interior_solid_array_for_porous_media: porous_media_internal.into(),
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            convection_heat_transfer_area_insulation_to_ambient,
            nusselt_correlation_fluid_to_porous_media_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
        };
    }

    /// constructs the static mixer pipe using the RELAP/SAM model 
    /// as a basis 
    ///
    /// Static mixer pipe 25a adjacent to MX-21
    /// in DHX branch
    /// pipe 25a
    /// otherwise known as the static mixer pipe 25a
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_pipe_25a_mx21(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.22245);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let form_loss = Ratio::new::<ratio>(1.35);



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );
        // for the porous media internal I am using the twisted 
        // tape dimensions as an estimate
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = component_length;

        let porous_media_internal = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
            


        // for new code, I need a lot of lengthscales and area scales 

        // so first, areas, usuall PI * d * l
        //
        // note that we don't actually have heat transfer areas of 
        // the interior for static mixers, so I'm just using a guestimate
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area
            = PI * steel_id * component_length;

        let convection_heat_transfer_area_fluid_to_interior: Area
            = convection_heat_transfer_area_fluid_to_pipe_shell * 2.0;

        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * fiberglass_od * component_length;

        // now nusselt correlations
        // I'm also guestimating that it is for the inside,
        // there was no experimental data or whatsover.
        let nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation 
            = therminol_array.nusselt_correlation.clone();

        let nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation 
            = nusselt_correlation_fluid_to_pipe_shell.clone();

        // now for the nusselt correlation lengthscales

        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let fiberglass_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let steel_shell_mid_diameter: Length = (steel_od + steel_id)/2.0;

        let fiberglass_shell_mid_diameter: Length 
            = (fiberglass_od + fiberglass_id)/2.0;


        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_id,
                steel_shell_mid_diameter,
                component_length,
                steel_thermal_conductivity).unwrap();


        let steel_shell_conductance_to_insulation_pipe_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_od,
                component_length,
                steel_thermal_conductivity).unwrap();

        let insulation_conductance_to_insulation_pipe_boundary: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_id,
                fiberglass_shell_mid_diameter,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let insulation_conductance_to_ambient: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_shell_mid_diameter,
                fiberglass_od,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length 
            = hydraulic_diameter;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length 
            = steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length 
            = steel_shell_conductance_to_insulation_pipe_boundary/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length 
            = insulation_conductance_to_insulation_pipe_boundary/fiberglass_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_ambient: Length 
            = insulation_conductance_to_ambient/fiberglass_thermal_conductivity;

        // for thermal conductance lengthscale for inner array,
        // I am guestimating
        //
        // just ignoring the thermal resistance of the 
        // insides, like lumped capacitance essentially

        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);

        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length 
            = hydraulic_diameter;

        // K = 1.8 in a pipe
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_pipe(
                component_length,
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(),
                hydraulic_diameter,
                form_loss
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            interior_solid_array_for_porous_media: porous_media_internal.into(),
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            convection_heat_transfer_area_insulation_to_ambient,
            nusselt_correlation_fluid_to_porous_media_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
        };
    }

    /// constructs the static mixer pipe using the RELAP/SAM model 
    /// as a basis 
    ///
    /// static mixer pipe 23a in DHX branch in CIET
    ///
    /// otherwise known as the static mixer pipe 
    /// to MX-20
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_pipe_23a_mx20(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.0891);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let form_loss = Ratio::new::<ratio>(1.35);



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );
        // for the porous media internal I am using the twisted 
        // tape dimensions as an estimate
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = component_length;

        let porous_media_internal = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
            


        // for new code, I need a lot of lengthscales and area scales 

        // so first, areas, usuall PI * d * l
        //
        // note that we don't actually have heat transfer areas of 
        // the interior for static mixers, so I'm just using a guestimate
        let convection_heat_transfer_area_fluid_to_pipe_shell: Area
            = PI * steel_id * component_length;

        let convection_heat_transfer_area_fluid_to_interior: Area
            = convection_heat_transfer_area_fluid_to_pipe_shell * 2.0;

        let convection_heat_transfer_area_insulation_to_ambient: Area 
            = PI * fiberglass_od * component_length;

        // now nusselt correlations
        // I'm also guestimating that it is for the inside,
        // there was no experimental data or whatsover.
        let nusselt_correlation_fluid_to_pipe_shell: NusseltCorrelation 
            = therminol_array.nusselt_correlation.clone();

        let nusselt_correlation_fluid_to_porous_media_interior: NusseltCorrelation 
            = nusselt_correlation_fluid_to_pipe_shell.clone();

        // now for the nusselt correlation lengthscales

        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let fiberglass_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::Fiberglass.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();

        let steel_shell_mid_diameter: Length = (steel_od + steel_id)/2.0;

        let fiberglass_shell_mid_diameter: Length 
            = (fiberglass_od + fiberglass_id)/2.0;


        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_id,
                steel_shell_mid_diameter,
                component_length,
                steel_thermal_conductivity).unwrap();


        let steel_shell_conductance_to_insulation_pipe_boundary: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_od,
                component_length,
                steel_thermal_conductivity).unwrap();

        let insulation_conductance_to_insulation_pipe_boundary: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_id,
                fiberglass_shell_mid_diameter,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let insulation_conductance_to_ambient: ThermalConductance =
            try_get_thermal_conductance_annular_cylinder(
                fiberglass_shell_mid_diameter,
                fiberglass_od,
                component_length,
                fiberglass_thermal_conductivity).unwrap();

        let nusselt_correlation_lengthscale_fluid_to_pipe_shell: Length 
            = hydraulic_diameter;

        let thermal_conductance_lengthscale_pipe_shell_to_fluid: Length 
            = steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface: Length 
            = steel_shell_conductance_to_insulation_pipe_boundary/steel_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface: Length 
            = insulation_conductance_to_insulation_pipe_boundary/fiberglass_thermal_conductivity;

        let thermal_conductance_lengthscale_insulation_to_ambient: Length 
            = insulation_conductance_to_ambient/fiberglass_thermal_conductivity;

        // for thermal conductance lengthscale for inner array,
        // I am guestimating
        //
        // just ignoring the thermal resistance of the 
        // insides, like lumped capacitance essentially

        let thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);

        let nusselt_correlation_lengthscale_fluid_to_porous_media_interior: Length 
            = hydraulic_diameter;

        // K = 1.8 in a pipe
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_pipe(
                component_length,
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(),
                hydraulic_diameter,
                form_loss
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            interior_solid_array_for_porous_media: porous_media_internal.into(),
            thermal_conductance_lengthscale_pipe_shell_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_pipe_shell_to_fluid,
            thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            thermal_conductance_lengthscale_insulation_to_insulation_pipe_interface,
            thermal_conductance_lengthscale_insulation_to_ambient,
            nusselt_correlation_fluid_to_pipe_shell,
            nusselt_correlation_lengthscale_fluid_to_pipe_shell,
            convection_heat_transfer_area_insulation_to_ambient,
            nusselt_correlation_fluid_to_porous_media_interior,
            nusselt_correlation_lengthscale_fluid_to_porous_media_interior,
            convection_heat_transfer_area_fluid_to_pipe_shell,
            convection_heat_transfer_area_fluid_to_interior,
        };
    }
}




/// contains method implementations for obtaining conductances 
/// between the different arrays, and also laterally coupling 
/// the arrays to one another using a radial thermal resistance
pub mod preprocessing;

/// contains method implementations for FluidComponentTrait
/// This means all the stuff about getting mass flowrate from pressure 
/// and vice versa
pub mod fluid_entity;

/// contains methods to help advance timesteps (ie update the 
/// state of the control volumes after each timestep)
pub mod calculation;

/// for postprocessing, one can obtain temperature profiles 
/// of the component using the postprocessing modules
pub mod postprocessing;

/// tests for regression, make sure conductances are 
/// calculated correctly
#[cfg(test)]
pub mod tests;
