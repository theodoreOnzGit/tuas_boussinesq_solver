/// calculates the courant number for a fluid
/// given a one dimensional model
use uom::si::{f64::*, volume_rate::cubic_meter_per_second};

/// calculates Courant-Friedrichs-Lewy number (CFL)
/// for 1D cells for fluid flow
///
/// formula is U * Delta T / Length
pub fn get_fluid_courant_number_one_dimension(
    fluid_velocity: Velocity,
    timestep: Time,
    cell_length_scale: Length
    ) -> Result<f64,f64> {

    let courant_number: Ratio = fluid_velocity
        *timestep/
        cell_length_scale;

    // if courant number greater than 1, return an error value
    // otherwise return an ok

    if courant_number.value > 1_f64 {
        return Err(courant_number.value);
    }

    return Ok(courant_number.value);

}

/// courant number 3D
///
/// calculates the CFL number for a 3D case
///
/// this is based on OpenFOAM's Courant number algorithm:
///
/// Co =  0.5 * timestep / volume *  
/// (summation (dot product of U and Area).abs())
///
/// Co =  0.5 * timestep / volume *  
/// (summation (dot product of U and normal vector).abs()*Area_magnitude)
/// 
/// this takes in three vectors, the volume of the control volume
/// and a timestep
///
/// This is for an arbitrarily shaped control volume with a number
/// of flat faces
///
/// the first vector specifies fluid velocity at each of these flat
/// faces
///
/// the second vector specifies the angle to the normal that these
/// velocities make with the area normals
/// we can define area normals as pointing inwards towards the
/// center of the cell, 
///
/// but it doesn't really matter, we need to take the absolute value of
/// the dot product of the velocities with the normals anyhow
///
// Here is OpenFoam's courant number file in C
// I take inspiration from this code,
// and it is GPL 3.0 anyway
//*---------------------------------------------------------------------------*\
//  =========                 |
//  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
//   \\    /   O peration     |
//    \\  /    A nd           | www.openfoam.com
//     \\/     M anipulation  |
//-------------------------------------------------------------------------------
//    Copyright (C) 2013-2016 OpenFOAM Foundation
//    Copyright (C) 2020 OpenCFD Ltd.
//-------------------------------------------------------------------------------
//License
//    This file is part of OpenFOAM.
//
//    OpenFOAM is free software: you can redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    OpenFOAM is distributed in the hope that it will be useful, but WITHOUT
//    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//    for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with OpenFOAM.  If not, see <http://www.gnu.org/licenses/>.
//
//\*---------------------------------------------------------------------------*/
//
//#include "CourantNo.H"
//#include "surfaceFields.H"
//#include "fvcSurfaceIntegrate.H"
//#include "zeroGradientFvPatchFields.H"
//#include "addToRunTimeSelectionTable.H"
//
//// * * * * * * * * * * * * * * Static Data Members * * * * * * * * * * * * * //
//
//namespace Foam
//{
//namespace functionObjects
//{
//    defineTypeNameAndDebug(CourantNo, 0);
//    addToRunTimeSelectionTable(functionObject, CourantNo, dictionary);
//}
//}
//
//
//// * * * * * * * * * * * * * Private Member Functions  * * * * * * * * * * * //
//
//Foam::tmp<Foam::volScalarField::Internal>
//Foam::functionObjects::CourantNo::byRho
//(
//    const tmp<volScalarField::Internal>& Co
//) const
//{
//    if (Co().dimensions() == dimDensity)
//    {
//        return Co/obr_.lookupObject<volScalarField>(rhoName_);
//    }
//
//    return Co;
//}
//
//
//bool Foam::functionObjects::CourantNo::calc()
//{
//    if (foundObject<surfaceScalarField>(fieldName_))
//    {
//        const surfaceScalarField& phi =
//            lookupObject<surfaceScalarField>(fieldName_);
//
//        tmp<volScalarField::Internal> Coi
//        (
//            byRho
//            (
//                (0.5*mesh_.time().deltaT())
//               *fvc::surfaceSum(mag(phi))()()
//               /mesh_.V()
//            )
//        );
//
//        if (foundObject<volScalarField>(resultName_, false))
//        {
//            volScalarField& Co =
//                lookupObjectRef<volScalarField>(resultName_);
//
//            Co.ref() = Coi();
//            Co.correctBoundaryConditions();
//        }
//        else
//        {
//            auto tCo = tmp<volScalarField>::New
//            (
//                IOobject
//                (
//                    resultName_,
//                    mesh_.time().timeName(),
//                    mesh_,
//                    IOobject::NO_READ,
//                    IOobject::NO_WRITE
//                ),
//                mesh_,
//                dimensionedScalar(dimless, Zero),
//                zeroGradientFvPatchScalarField::typeName
//            );
//            tCo.ref().ref() = Coi();
//            tCo.ref().correctBoundaryConditions();
//            mesh_.objectRegistry::store(tCo.ptr());
//        }
//
//        return true;
//    }
//
//    return false;
//}
//
//
//// * * * * * * * * * * * * * * * * Constructors  * * * * * * * * * * * * * * //
//
//Foam::functionObjects::CourantNo::CourantNo
//(
//    const word& name,
//    const Time& runTime,
//    const dictionary& dict
//)
//:
//    fieldExpression(name, runTime, dict, "phi"),
//    rhoName_("rho")
//{
//    setResultName("Co", "phi");
//    read(dict);
//}
//
//
//// * * * * * * * * * * * * * * * Member Functions  * * * * * * * * * * * * * //
//
//bool Foam::functionObjects::CourantNo::read(const dictionary& dict)
//{
//    fieldExpression::read(dict);
//
//    rhoName_ = dict.getOrDefault<word>("rho", "rho");
//
//    return true;
//}
//
//
//// ************************************************************************* //
pub fn courant_number_3d_openfoam_algorithm_velocity(
    // pardon the naming convention,
    // control_volume_volume
    // 
    // means how much volume the control volume occupies
    control_volume_volume: Volume,
    timestep: Time,
    face_area_vector: Vec<Area>,
    velocity_vector: Vec<Velocity>,
    angle_between_area_normals_and_velocity_vector: Vec<Angle>,
    ) -> Result<f64,f64> {

    // first we need to check if the length of each vector is the same

    if velocity_vector.len() != face_area_vector.len() {
        return Err(f64::NAN);
    }

    if velocity_vector.len() !=  angle_between_area_normals_and_velocity_vector.len() {
        return Err(f64::NAN);
    }

    // first we calculate the cosine of the angles
    //

    let mut cosine_angle_vector: Vec<f64> = vec![];

    for angle in angle_between_area_normals_and_velocity_vector.iter() {

        // first get angle in radians
        let angle_value_radians: f64 = angle.value;

        // we get the cosine

        let angle_cosine = angle_value_radians.cos();

        cosine_angle_vector.push(angle_cosine);
    }

    // then let's calculate the dot product
    // it has units of volumetric flowrate
    let mut dot_product: VolumeRate
        = VolumeRate::new::<cubic_meter_per_second>(0.0);

    for (index, velocity_pointer) in velocity_vector.iter().enumerate() {

        // we get the velocity
        // multiplied by the cosine and area
        // then take absolute value
        //

        let angle_cosine = cosine_angle_vector[index];
        let face_area = face_area_vector[index];

        // note that i ha
        let volumetric_flow_rate: VolumeRate = 
            face_area 
            * (*velocity_pointer)
            * angle_cosine;
        
        let abs_volumetric_flowrate = 
            volumetric_flow_rate.abs();

        
        dot_product += abs_volumetric_flowrate;

    }

    // Co =  0.5 * timestep / volume *  
    // (summation (dot product of U and normal vector).abs()*Area_magnitude)
    let courant_number: Ratio = 
        0.5 
        * timestep
        / control_volume_volume
        * dot_product;

    // i'll return an error value if the courant number is below zero
    if courant_number.value > 1_f64 {
        return Err(courant_number.value);
    }

    return Ok(courant_number.value);

}

/// similar algorithm based on volumetric flowrates in and out
pub fn courant_number_3d_openfoam_algorithm_vol_flowrate(
    // pardon the naming convention,
    // control_volume_volume
    // 
    // means how much volume the control volume occupies
    control_volume_volume: Volume,
    timestep: Time,
    volume_flowrate_vector: Vec<VolumeRate>
    ) -> Result<f64,f64> {


    // then let's calculate the dot product
    // it has units of volumetric flowrate
    let mut absolute_sum_of_volumetric_flowrate: VolumeRate
        = VolumeRate::new::<cubic_meter_per_second>(0.0);

    for vol_flowrate_ptr in volume_flowrate_vector.iter() {

        absolute_sum_of_volumetric_flowrate += vol_flowrate_ptr.abs();

    }

    // Co =  0.5 * timestep / volume *  
    // (summation (dot product of U and normal vector).abs()*Area_magnitude)
    let courant_number: Ratio = 
        0.5 
        * timestep
        / control_volume_volume
        * absolute_sum_of_volumetric_flowrate;

    // i'll return an error value if the courant number is below zero
    if courant_number.value > 1_f64 {
        return Err(courant_number.value);
    }

    return Ok(courant_number.value);

}
// courant number for energy?
//
// the timescale for such things is the time needed
// to get the system to a steady state value
//
// if the system reaches steady state in less than
// one timestep, i think the courant number is exceeded
//
// this is just intuition however, probably need to prove it
//
// one other way to think about courant number is knowing the
// courant number expression for conduction
// and then derive from there
//
// 
//
/// Courant number equivalent for conduction heat transfer
///
/// For conduction based heat transfer,
/// Courant number is just the fourier number
/// but the characteristic length is 
/// the mesh length
///
/// Co = Fo
/// fourier number
///
/// will return an error if value is more than 0.25
///
pub fn fourier_number_heat_conduction(
    alpha_thermal_diffusivity: DiffusionCoefficient,
    timestep: Time,
    mesh_length: Length) -> Result<f64, f64>{

    let courant_number = alpha_thermal_diffusivity *
        timestep
        / mesh_length
        / mesh_length;


    if courant_number.value > 0.25_f64 {
        return Err(courant_number.value);
    }

    return Ok(courant_number.value);

}

/// Courant number equivalent for convection heat transfer
/// essentially calculates Co = Bi Fo
///
/// will return an error if value is more than 0.25
///
pub fn fourier_number_heat_convection(
    heat_transfer_coeffcient_for_external_fluid: HeatTransfer,
    control_volume_thermal_conductivity: ThermalConductivity,
    volume_cv_to_surface_area_cv_ratio: Length,
    alpha_thermal_diffusivity: DiffusionCoefficient,
    timestep: Time,
    ) -> Result<f64,f64>{

    let biot_number: Ratio = heat_transfer_coeffcient_for_external_fluid
        *volume_cv_to_surface_area_cv_ratio
        /control_volume_thermal_conductivity;

    let fourier_number: Ratio = alpha_thermal_diffusivity 
        *timestep
        /volume_cv_to_surface_area_cv_ratio
        /volume_cv_to_surface_area_cv_ratio;

    let courant_number:Ratio = biot_number*fourier_number;

    if courant_number.value > 0.25_f64 {
        return Err(courant_number.value);
    }

    return Ok(courant_number.value);

}

/// Courant number for heat transport (ie mass flowrate)
///
/// calculate courant number for enthalpy transport based
/// on mass flowrates on one face
pub fn single_face_courant_number_enthalpy_flow(
    mass_flowrate: MassRate,
    control_volume_mass: Mass,
    timestep: Time,) -> Result<f64,f64> {

    let courant_number: Ratio = 0.5 * mass_flowrate.abs() 
        * timestep
        / control_volume_mass;

    if courant_number.value > 0.25_f64 {
        return Err(courant_number.value);
    }

    return Ok(courant_number.value);
}

