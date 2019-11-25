// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

'use strict'

const {createHash} = require('crypto')
const {CryptoFactory, createContext } = require('sawtooth-sdk/signing')
const protobuf = require('sawtooth-sdk/protobuf')
const fs = require('fs')
const fetch = require('node-fetch');
const {TextEncoder, TextDecoder} = require('text-encoding/lib/encoding')

//const FAMILY_NAME='dgc-core'

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

class dgcRequest {
  constructor(privateKeyStr) {
    const privateKey = Secp256k1PrivateKey.fromHex(privateKeyStr);
    this.signer = new CryptoFactory(context).newSigner(privateKey);
    this.publicKey = this.signer.getPublicKey().asHex();
    this.address = hash("dgc-core").substr(0, 6) + hash(this.publicKey).substr(0, 64);
    console.log("Storing at: " + this.address);  
  }

  isPrivateKey() {
    return this._send_to_rest_api(null);
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
    return this._send_to_rest_api(null);
  }

  dgcCredit() {
    return this._send_to_rest_api(null);
  } //imcomplete

  transferDGC(amount, user2) {
    this._wrap_and_send("transfer", [amount, user2]);
  }

  dgcExchange(currency) {
    return this._send_to_rest_api(null);
  } //imcomplete

  sellDGC(dgc_amount, currency, currency_amount) {
    this._wrap_and_send("sellDGC", [dgc_amount, currency, currency_amount]);
  }

  buyDGC(dgc_amount, currency, currency_amount) {
    this._wrap_and_send("buyDGC", [dgc_amount, currency, currency_amount]);
  }

/*
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
*/
  _wrap_and_send(action,values){
    var payload = ''
    const address = this.address;
    console.log("wrapping for: " + this.address);
    var inputAddressList = [address];
    var outputAddressList = [address];
    if (action === "transfer") {
      console.log(values[1]);
	    const pubKeyStr = values[1];
      var toAddress = hash("dgc-core").substr(0, 6) + hash(pubKeyStr).substr(0, 64);
      inputAddressList.push(toAddress);
      outputAddressList.push(toAddress);
      payload = action+","+values[0]+","+pubKeyStr;
    } else {
	    payload = action+","+values[0];
    }	
    var enc = new TextEncoder('utf8');
    const payloadBytes = enc.encode(payload);
    const transactionHeaderBytes = protobuf.TransactionHeader.encode({
      familyName: 'dgc-core',
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
    //this._send_to_rest_api(batchListBytes);	
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

  _send_to_rest_api(batchListBytes){
    if (batchListBytes == null) {
      var geturl = 'http://rest-api:8008/state/'+this.address
      console.log("Getting from: " + geturl);
      return fetch(geturl, {
        method: 'GET',
      })
      //.catch((error) => {
      //  console.error(error);
      //  return false;
      //})
      .then((response) => response.json())
      .then((responseJson) => {
        var data = responseJson.data;
        if (null == data) return 0
        else {
          var amount = new Buffer(data, 'base64').toString();
          return amount;  
        }
      })
      .catch((error) => {
        console.error(error);
        return false;
      }); 	
    } else {
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
module.exports.dgcRequest = dgcRequest;
