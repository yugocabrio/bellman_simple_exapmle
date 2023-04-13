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

struct MultiplyDemo<Scalar: PrimeField> {
    a: Option<Scalar>,
    b: Option<Scalar>,
    c: Option<Scalar>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for MultiplyDemo<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate the first value (private)
        let a = cs.alloc(|| "a", || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the second value (private)
        let b = cs.alloc(|| "b", || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the third value (public)
        // allocating a public input uses alloc_input
        let c = cs.alloc_input(|| "c", || {
            self.c.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // a * b = c?
        cs.enforce(
            || "mult",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c
        );
        
        Ok(())
    }
}

fn main() {
    let rng = &mut OsRng;

    let pk = {
        let c = MultiplyDemo {
            a: None,
            b: None,
            c: None,
        };

        groth16::generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = groth16::prepare_verifying_key(&pk.vk);

    let assignment = MultiplyDemo {
        a: Some(4.into()),
        b: Some(2.into()),
        c: Some(8.into()),
    };

    let public_inputs = vec![assignment.c.unwrap()];

    let proof = groth16::create_random_proof(assignment, &pk, rng).unwrap();

    let verification_result = groth16::verify_proof(&pvk, &proof, &public_inputs);

    match verification_result {
        Ok(_) => println!("Proof verification: SUCCESS"),
        Err(_) => println!("Proof verification: FAILED"),
    }
}

