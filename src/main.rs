use backwards::{SequenceValidator, SequenceGenerator};

fn main() {
    let seq_val = SequenceValidator::new(5, 1);
    let seq_gen = SequenceGenerator::new();

    let num_seq_to_try: usize = 1_000;

    for _ in 0..num_seq_to_try {
        let sequence = seq_gen.generate_sequence(10);
        let status = seq_val.validate(sequence.clone());

        if status.is_valid && status.exceptions > 0 {
            println!("Found an example with {:?} exceptions: {:?}", status.exceptions, sequence);
        }
    }
}