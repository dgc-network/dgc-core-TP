// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

const {createHash} = require('crypto')
const {CryptoFactory, createContext } = require('sawtooth-sdk/signing')
const protobuf = require('sawtooth-sdk/protobuf')
const fs = require('fs')
const fetch = require('node-fetch');
const {TextEncoder, TextDecoder} = require('text-encoding/lib/encoding')

FAMILY_NAME='dgc-wallet'

function hash(v) {
  return createHash('sha512').update(v).digest('hex');
}

/**
 * Generates a new private key, saving it to memory and storage (encrypted).
 * Returns both a public key and the encrypted private key.
 */
const {Secp256k1PrivateKey} = require('sawtooth-sdk/signing/secp256k1')	
const secp256k1 = require('sawtooth-sdk/signing/secp256k1')
const context = new secp256k1.Secp256k1Context()
//const context = createContext('secp256k1');
//let privateKey = null
//let publicKey = null
//let encryptedKey = null


class dgcWalletRequest {
  //constructor(userid) {
    //const privateKeyStrBuf = this.getUserPriKey(userid);
    //const privateKeyStr = privateKeyStrBuf.toString().trim();
  constructor(privateKeyStr) {
    if (null == privateKeyStr)
      console.log("privateKey is empty");
    else {
      const privateKey = Secp256k1PrivateKey.fromHex(privateKeyStr);
      this.signer = new CryptoFactory(context).newSigner(privateKey);
      this.publicKey = this.signer.getPublicKey().asHex();
      this.address = hash("dgc-wallet").substr(0, 6) + hash(this.publicKey).substr(0, 64);
      console.log("Storing at: " + this.address);  
    }
  }

  makePrivateKey() {
    let privateKey = context.newRandomPrivateKey()
    privateKey = privateKey.asHex()
    return privateKey
  }

  getPublicKey() {
    return this.publicKey
  }

  dgcBalance() {
    let amount = this._send_to_rest_api(null);
    return amount;
  }

  dgcTransfer(user2, amount) {
    this._wrap_and_send("transfer", [amount, user2]);
  }

  deposit(amount) {
    this._wrap_and_send("deposit", [amount]);
  }

  withdraw(amount) {
    this._wrap_and_send("withdraw", [amount]);
  }	

  balance() {
    let amount = this._send_to_rest_api(null);
    return amount;
  }

  transfer(user2, amount) {
    this._wrap_and_send("transfer", [amount, user2]);
  }

  getUserPriKey(userid) {
    console.log(userid);
    console.log("Current working directory is: " + process.cwd());
    var userprivkeyfile = '/root/.sawtooth/keys/'+userid+'.priv';
    return fs.readFileSync(userprivkeyfile);
  }	

  getUserPubKey(userid) {
    console.log(userid);
    console.log("Current working directory is: " + process.cwd());
    var userpubkeyfile = '/root/.sawtooth/keys/'+userid+'.pub';
    return fs.readFileSync(userpubkeyfile);
  }

  _wrap_and_send(action,values){
    var payload = ''
    const address = this.address;
    console.log("wrapping for: " + this.address);
    var inputAddressList = [address];
    var outputAddressList = [address];
    if (action === "transfer") {
	    //const pubKeyStrBuf = this.getUserPubKey(values[1]);
      //const pubKeyStr = pubKeyStrBuf.toString().trim();
      console.log(values[1]);
	    const pubKeyStr = values[1];
      var toAddress = hash("dgc-wallet").substr(0, 6) + hash(pubKeyStr).substr(0, 64);
      inputAddressList.push(toAddress);
      outputAddressList.push(toAddress);
      payload = action+","+values[0]+","+pubKeyStr;
    } 
    else {
	    payload = action+","+values[0];
    }	
    var enc = new TextEncoder('utf8');
    const payloadBytes = enc.encode(payload);
    const transactionHeaderBytes = protobuf.TransactionHeader.encode({
      familyName: 'dgc-wallet',
      familyVersion: '1.0',
      inputs: inputAddressList,
      outputs: outputAddressList,
      signerPublicKey: this.signer.getPublicKey().asHex(),
      nonce: "" + Math.random(),
      batcherPublicKey: this.signer.getPublicKey().asHex(),
      dependencies: [],
      payloadSha512: hash(payloadBytes),
    }).finish();
    const transaction = protobuf.Transaction.create({
      header: transactionHeaderBytes,
      headerSignature: this.signer.sign(transactionHeaderBytes),
      payload: payloadBytes
    });
    const transactions = [transaction];
    const batchHeaderBytes = protobuf.BatchHeader.encode({
      signerPublicKey: this.signer.getPublicKey().asHex(),
      transactionIds: transactions.map((txn) => txn.headerSignature),
    }).finish();
    const batchSignature = this.signer.sign(batchHeaderBytes);
    const batch = protobuf.Batch.create({
      header: batchHeaderBytes,
      headerSignature: batchSignature,
      transactions: transactions,
    });
    const batchListBytes = protobuf.BatchList.encode({
      batches: [batch]
    }).finish();
    this._send_to_rest_api(batchListBytes);	
  }

  _send_to_rest_api(batchListBytes){
    if (batchListBytes == null) {
      var geturl = 'http://rest-api:8008/state/'+this.address
      console.log("Getting from: " + geturl);
      return fetch(geturl, {
        method: 'GET',
      })
      .then((response) => response.json())
      .then((responseJson) => {
        var data = responseJson.data;
        var amount = new Buffer(data, 'base64').toString();
        return amount;
      })
      .catch((error) => {
        console.error(error);
      }); 	
    }
    else{
      fetch('http://rest-api:8008/batches', {
 	      method: 'POST',
        headers: {
	        'Content-Type': 'application/octet-stream'
        },
        body: batchListBytes
	    })
	    .then((response) => response.json())
	    .then((responseJson) => {
        console.log(responseJson);
      })
      .catch((error) => {
 	      console.error(error);
      }); 	
    }
  }
}
module.exports.dgcWalletRequest = dgcWalletRequest;
