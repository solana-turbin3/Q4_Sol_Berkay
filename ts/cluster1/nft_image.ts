import wallet from './wallet/wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
	createGenericFile,
	createSignerFromKeypair,
	signerIdentity,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { readFile } from 'fs/promises';


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
		const file = 'generug.png';
		const buffer = await readFile(file);
		// Convert image to generic file
		const image = createGenericFile(buffer, 'image/png');
		// Upload image to bundlr
		const [imageURI] = await umi.uploader.upload([image]);
		console.log('Image URI uploaded-------------->', imageURI);
		return imageURI;
	} catch (error) {
		console.log(error);
	}
})();
