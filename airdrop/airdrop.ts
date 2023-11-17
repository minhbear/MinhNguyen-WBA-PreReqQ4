import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import wallet from './dev-wallet.json';

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection('https://api.devnet.solana.com');

(async () => {
	try {
		// We're going to claim 2 devnet SOL tokens
		const txhash = await connection.requestAirdrop(
			keypair.publicKey,
			2 * LAMPORTS_PER_SOL
		);
    // Success! Check out your TX here: https://explorer.solana.com/tx/2ZsVD1jpachU26q6CwNWTVDNwNSgjAoJoUhqGcv9zXz2xiUiPVf7rYhrrCsXiZSnJdtPmNDQ63azBR8t45b18uDc?cluster=devnet
		https: console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
