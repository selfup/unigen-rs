#[derive(Debug)]
pub struct Nucleus {
  pub protons: i8,
  pub neutrons: i8,
}

#[derive(Debug)]
pub struct Atom {
  pub electrons: i8,
  pub nucleus: Nucleus,
}
