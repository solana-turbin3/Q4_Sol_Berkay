import {
	Keypair,
	PublicKey,
	Connection,
	Commitment,
	clusterApiUrl,
} from '@solana/web3.js';
import {
	getOrCreateAssociatedTokenAccount,
	mintTo,
} from '@solana/spl-token';
import wallet from './wallet/wba-wallet.json';


// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(walletPk));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection(
	'http://127.0.0.1:8899',
	commitment
);

const token_decimals = 1_000_000;

// Mint address
const mint = new PublicKey(
	'DxhNFBqUkQr2DJ964aovMJBFwCFoe1vr8WDzjEyrUy7H'
);

(async () => {
	try {
		// Create an ATA
		const ata = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			keypair.publicKey
		);
		console.log(`Your ata is: ${ata.address.toBase58()}`);

		// Mint to ATA
		const mintTx = await mintTo(
			connection,
			keypair,
			mint,
			ata.address,
			keypair.publicKey,
			1000 * token_decimals
		);
		console.log(`Your mint txid: ${mintTx}`);
	} catch (error) {
		console.log(`Oops, something went wrong: ${error}`);
	}
})();
