use arc_ec::{ProjectiveCurve, AffineCurve, PairingEngine};
use ark_ff::{PrimeField, Field};
// we use BLS!@-381 pairing friendly group for this example 
use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, G1Affine, G2Affine, Fr as ScalarField };
use ark_std::{Zero, UniformRand};

let mut  rng = ark_std::rand::thread_rng();
// let sample uniformly   random field elements:
let a: G1Affine = G1::rand(&mut rng).into();
let b: G2Affine = G2::rand(&mut rng).into();

// we can compute the pairing of 'a' and 'b' 
let c =   Bls12_381::pairing(a, b);

// we can also compute the pairing partwise:
//first compute the miller loop:
let c_ml = Bls12_381::miller_loop(&[(a.into(), b.into())]);
let c_fe = Bls12_381::final_exponentiation(&c_ml).unwrap();
assert_eq(c, c_fe);

