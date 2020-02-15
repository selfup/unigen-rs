#[derive(Debug)]
pub struct Nucleus {
  pub protons: i16,
  pub neutrons: i16,
}

#[derive(Debug)]
pub struct Atom {
  pub electrons: i16,
  pub nucleus: Nucleus,
}
