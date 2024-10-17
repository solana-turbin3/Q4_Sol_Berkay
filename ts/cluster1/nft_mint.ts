import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
	createSignerFromKeypair,
	signerIdentity,
	generateSigner,
	percentAmount,
} from '@metaplex-foundation/umi';
import {
	createNft,
	mplTokenMetadata,
} from '@metaplex-foundation/mpl-token-metadata';

import wallet from './wallet/wallet.json';
import base58 from 'bs58';

const RPC_ENDPOINT = 'https://api.devnet.solana.com';
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(
	new Uint8Array(wallet)
);
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
	let tx = createNft(umi, {
		mint,
		authority: myKeypairSigner,
		name: 'Ruggish',
		symbol: 'RUG',
		sellerFeeBasisPoints: percentAmount(0.5),
		uri: 'https://arweave.net/FBurMJ1wZz89xicUEt89GrYpovqAb7i5mnfasdURmdem',
	});
	let result = await tx.sendAndConfirm(umi);
	const signature = base58.encode(result.signature);

	console.log(
		`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
	);

	console.log('Mint Address: ', mint.publicKey);
})();
