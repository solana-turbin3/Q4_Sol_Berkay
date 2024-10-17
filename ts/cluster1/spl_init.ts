import { Keypair, Connection, Commitment } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import wallet from './wallet/wallet.json';

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection(
	'http://127.0.0.1:8899',
	commitment
);

const signer = Keypair.fromSecretKey(new Uint8Array(walletPk));

(async () => {
	try {
		// Start here
		const mint = await createMint(
			connection,
			signer,
			signer.publicKey,
			null,
			6
		);
		console.log('Your mint address ---------->', mint.toBase58());
	} catch (error) {
		console.log(`Oops, something went wrong: ${error}`);
	}
})();
