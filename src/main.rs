use backwards::SequenceGenerator;

fn main() {
    let seq_gen = SequenceGenerator::new();

    let sequence = seq_gen.generate_sequence(10);

    println!("generated sequence: {:?}", sequence);
}