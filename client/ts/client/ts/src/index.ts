import * as anchor from "@coral-xyz/anchor";


export const ID = new anchor.web3.PublicKey("Fg1WhEel111111111111111111111111111111111");


export function program(provider: anchor.AnchorProvider) {
return new anchor.Program(require("./idl/solana_perps_flywheel.json"), ID, provider);
}
