// Step 0: From Entropy to Mnemonic
// Step 1: From Mnemonic to Seed
pub mod mnemonic;

// Step 2: From Seed to Master Extended Keypair
// Step 3: From Master Extended Keypair to Child Extended Keypair
pub mod derivation;

// Step 4: From Child Extended Keypair to Adresse Parameter
pub mod adressing;

// Step 5: Get Balance