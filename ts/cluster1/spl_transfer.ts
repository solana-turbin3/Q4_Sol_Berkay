import {
	Commitment,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
} from '@solana/web3.js';
// import wallet from '../wba-wallet.json';
import {
	getOrCreateAssociatedTokenAccount,
	transfer,
} from '@solana/spl-token';



// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(walletPk));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection(
	'http://127.0.0.1:8899',
	commitment
);

// Mint address
const mint = new PublicKey(
	'DxhNFBqUkQr2DJ964aovMJBFwCFoe1vr8WDzjEyrUy7H'
);

// Recipient address
// const to = new PublicKey('<receiver address>');

(async () => {
	try {
		// Get the token account of the fromWallet address, and if it does not exist, create it
		const fromATA = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			keypair.publicKey
		);
		// Get the token account of the toWallet address, and if it does not exist, create it
		const toATA = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			keypair.publicKey
		);

		// Transfer the new token to the "toTokenAccount" we just created
		const signature = await transfer(
			connection,
			keypair,
			fromATA.address,
			toATA.address,
			keypair,
			1e6
		);

		console.log(signature);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
