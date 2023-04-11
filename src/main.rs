use bellman::{
    gadgets::{
        boolean::{AllocatedBit, Boolean},
        num::AllocatedNum,
    },
    groth16, Circuit, ConstraintSystem, LinearCombination, SynthesisError,
};
use bls12_381::Bls12;
use ff::PrimeField;
use rand::rngs::OsRng;

// MyCircuit struct remains the same
struct MyCircuit<Scalar: PrimeField> {
    a: Option<Scalar>,
    b: Option<Scalar>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for MyCircuit<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate the variables a and b
        let a = AllocatedNum::alloc(cs.namespace(|| "a"), || {
            Ok(*self.a.as_ref().unwrap())
        })?;
        let b = AllocatedNum::alloc(cs.namespace(|| "b"), || {
            Ok(*self.b.as_ref().unwrap())
        })?;

        // Compute a * b
        let c = a.mul(cs.namespace(|| "a * b"), &b)?;

        // Enforce the constraint a * b = 24
        let twenty_four_lc = LinearCombination::zero() + (Scalar::from(24u64), CS::one());


        //let twenty_four_lc = ConstraintSystem::<Scalar>::one() * Scalar::from(24u64);
        cs.enforce(
            || "a * b = 24",
            |lc| lc + a.get_variable(),
            |lc| lc + b.get_variable(),
            |lc| lc + c.get_variable() - &twenty_four_lc,
        );
        

        Ok(())
    }
}


fn main() {
    // Use the same MyCircuit definition from the previous post.

    // Create parameters for our circuit. In a production deployment these would
    // be generated securely using a multiparty computation.
    let params = {
        let c = MyCircuit::<bls12_381::Scalar> { a: None, b: None };
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()
    };

    // Prepare the verification key (for proof verification).
    let pvk = groth16::prepare_verifying_key(&params.vk);

    // Choose values for a and b, such that a * b = 24.
    let a_value = bls12_381::Scalar::from(3u64);
    let b_value = bls12_381::Scalar::from(8u64);

    // Create an instance of our circuit (with a and b as witnesses).
    let c = MyCircuit {
        a: Some(a_value),
        b: Some(b_value),
    };

    // Create a Groth16 proof with our parameters.
    let proof = groth16::create_random_proof(c, &params, &mut OsRng).unwrap();

    // Compute inputs for proof verification.
    // let inputs = {vec![bls12_381::Scalar::from(24u64)]};
    let public_inputs = {
        vec![bls12_381::Scalar::from(24u64)]
    };

    // Check the proof!
    let verification_result = groth16::verify_proof(&pvk, &proof, &public_inputs);

    match verification_result {
        Ok(_) => println!("Proof verification: SUCCESS"),
        Err(_) => println!("Proof verification: FAILED"),
    }
}