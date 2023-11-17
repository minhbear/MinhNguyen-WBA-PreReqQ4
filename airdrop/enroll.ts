import {
	Address,
	AnchorProvider,
	Program,
	Wallet,
} from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { IDL, WbaPrereq } from './programs/wba_prereq';
import wallet from './wba_wallet.json';

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection('https://api.devnet.solana.com');
const github = Buffer.from('MinhNguyen', 'utf8');
const provider = new AnchorProvider(connection, new Wallet(keypair), {
	commitment: 'confirmed',
});
const program = new Program<WbaPrereq>(
	IDL,
	'HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1' as Address,
	provider
);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from('prereq'), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
	enrollment_seeds,
	program.programId
);

// Execute our enrollment transaction
(async () => {
	try {
		const txhash = await program.methods
			.complete(github)
			.accounts({
				signer: keypair.publicKey,
				prereq: enrollment_key,
				systemProgram: SystemProgram.programId,
			})
			.signers([keypair])
			.rpc();
    // Success! Check out your TX here: https://explorer.solana.com/tx/5cCaGbtRQ1F8evkj1d1NSco1DMP6U7KBu9qpKeZa5N7mxVcBc1VdvZQ6DTEidRjRCx5crbLwtFshw7bwx7cjfteJ?cluster=devnet
		console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
