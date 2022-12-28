use std::fs::File;
use std::io::{Cursor, Read, Write};

use clap::Parser;

use rln::{
    protocol::{
        proof_values_from_witness, random_rln_witness, serialize_proof_values, serialize_witness,
    },
    public::RLN,
};

mod args;
use args::*;

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::GenerateContract => println!("ava"),
        Commands::GenerateProof { path } => {
            let tree_height = 20;
            let resources = Cursor::new("./temp/");

            let mut rln = RLN::new(tree_height, resources);

            let rln_witness = random_rln_witness(tree_height);

            let mut input_buffer = Cursor::new(serialize_witness(&rln_witness));
            let mut output_buffer = Cursor::new(Vec::<u8>::new());

            rln.prove(&mut input_buffer, &mut output_buffer).unwrap();
            let zk_proof = output_buffer.into_inner();

            let proof_values = proof_values_from_witness(&rln_witness);
            let serialized_proof_values = serialize_proof_values(&proof_values);

            let mut verify_data = Vec::<u8>::new();
            verify_data.extend(&zk_proof);
            verify_data.extend(&serialized_proof_values);

            File::create(path).unwrap().write_all(&verify_data).unwrap();
        }
        Commands::VerifyProof { path } => {
            let tree_height = 20;
            let resources = Cursor::new("./temp/");

            let mut verify_data = Vec::<u8>::new();
            File::open(path)
                .unwrap()
                .read_to_end(&mut verify_data)
                .unwrap();

            let mut input_buffer = Cursor::new(verify_data);

            let rln = RLN::new(tree_height, resources);

            let verified = rln.verify(&mut input_buffer).unwrap();
            assert!(verified);
        }
    };
}
