import wallet from './wallet/wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
	createGenericFile,
	createSignerFromKeypair,
	signerIdentity,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(
	new Uint8Array(wallet)
);
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));


(async () => {
	try {
		const image =
			' https://arweave.net/3fg84jX1zb7ZPMhfTkimxZ6wDphqsj9Sq1SS4wr6tCb6';
		const metadata = {
			name: 'Ruggish',
			symbol: 'RUG',
			description:
				'That is a traditional ai generated ruggish, aka rug.',
			image,
			attributes: [{ trait_type: 'rarirty', value: 'legendary' }],
			properties: {
				files: [
					{
						type: 'image/png',
						uri: image,
					},
				],
			},
			creators: [],
		};

		const myUri = await umi.uploader.uploadJson(metadata);
		console.log('Your image URI: ', myUri);
	} catch (error) {
		console.log('Oops.. Something went wrong', error);
	}
})();
