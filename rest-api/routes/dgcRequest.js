// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

'use strict'

const {createHash} = require('crypto')
const {CryptoFactory, createContext } = require('sawtooth-sdk/signing')
const {Secp256k1PrivateKey} = require('sawtooth-sdk/signing/secp256k1')
const protobuf = require('sawtooth-sdk/protobuf')
//const fs = require('fs')
const fetch = require('node-fetch');
const {TextEncoder, TextDecoder} = require('text-encoding/lib/encoding')

const context = createContext('secp256k1')

const FAMILY_NAME = "dgc-core"
const FAMILY_VER = "1.0"
const DGC_BALANCE = "balance"
const DGC_CREDIT  = "credit"
const DGC_EXCHANGE= "exchange"
const APPLY_CREDIT = "apply"
const TRANSFER_DGC = "transfer"
const SELL_DGC = "sell"
const BUY_DGC = "buy"

function hash(v) {
  return createHash('sha512').update(v).digest('hex');
}

function make_balance_address(identifier) {
  return hash(FAMILY_NAME).substr(0, 6) + hash(DGC_BALANCE).substr(0, 2) + hash(identifier).substr(0, 62);
}

function make_credit_address(identifier, currency) {
  return hash(FAMILY_NAME).substr(0, 6) + hash(DGC_CREDIT).substr(0, 2) + hash(currency).substr(0, 2) + hash(identifier).substr(0, 60);
}

function make_exchange_address(currency) {
  return hash(FAMILY_NAME).substr(0, 6) + hash(DGC_EXCHANGE).substr(0, 2) + hash(currency).substr(0, 62);
}

class dgcRequest {
  constructor(privateKeyHex) {
    if (undefined !== privateKeyHex) {
      const privateKey = Secp256k1PrivateKey.fromHex(privateKeyHex);
      this.signer = new CryptoFactory(context).newSigner(privateKey)
      this.publicKeyHex = this.signer.getPublicKey().asHex();
    }
  }

  isPrivateKey() {
    //return this._send_to_rest_api(null);
  }

  makePrivateKey() {
    const privateKey = context.newRandomPrivateKey()
    const privateKeyHex = privateKey.asHex()
    return privateKeyHex
  }

  getPublicKey() {
    return this.publicKeyHex
  }

  dgcBalance() {
    return this._get_from_rest_api(DGC_BALANCE);
  }

  dgcCredit(currency='DGC') {
    return this._get_from_rest_api(DGC_CREDIT, currency);
  }

  dgcExchange(currency) {
    return this._get_from_rest_api(DGC_EXCHANGE, currency);
  }

  _get_from_rest_api(action, value){
    let address = '';
    if (action == DGC_BALANCE) {
      address = make_balance_address(this.publicKeyHex);
    } else if (action == DGC_CREDIT) {
      address = make_credit_address(this.publicKeyHex, value);
    } else if (action == DGC_EXCHANGE) {
      address = make_exchange_address(value);
    }
    console.log("Storing at: " + address);
    const geturl = 'http://rest-api:8008/state/'+address
    console.log("Getting from: " + geturl);
    return fetch(geturl, {
      method: 'GET',
    })
    .then((response) => response.json())
    .then((responseJson) => {
      console.log(responseJson);
      return responseJson;
    })
    .catch((error) => {
      console.error(error);
    });
  }

  applyCredit(amount, currency) {
    return this._post_to_rest_api(APPLY_CREDIT, [amount, currency]);
  }

  transferDGC(amount, user2) {
    return this._post_to_rest_api(TRANSFER_DGC, [amount, user2]);
  }

  sellDGC(dgc_amount, currency, expected_currency_amount) {
    return this._post_to_rest_api(SELL_DGC, [dgc_amount, currency, expected_currency_amount]);
  }

  buyDGC(dgc_amount, currency, expected_currency_amount) {
    return this._post_to_rest_api(BUY_DGC, [dgc_amount, currency, expected_currency_amount]);
  }

  _post_to_rest_api(action, values){
    let payload = ''
    let inputAddressList = [address];
    let outputAddressList = [address];

    if (action === APPLY_CREDIT) {
      const currency = values[1];
      const address = make_credit_address(this.publicKeyHex, currency);
      inputAddressList.push(address);
      outputAddressList.push(address);
      console.log("wrapping for: " + address);
      payload = action+","+values[0]+","+currency;

    } else if (action === TRANSFER_DGC) {
      const address = make_balance_address(this.publicKeyHex);
      inputAddressList.push(address);
      outputAddressList.push(address);
      console.log("wrapping for: " + address);
      const pubKeyStr = values[1];
      const toAddress = make_balance_address(pubKeyStr);
      inputAddressList.push(toAddress);
      outputAddressList.push(toAddress);
      payload = action+","+values[0]+","+pubKeyStr;

    } else if (action === SELL_DGC) {
      const address = make_balance_address(this.publicKeyHex);
      inputAddressList.push(address);
      outputAddressList.push(address);
      console.log("wrapping for: " + address);
      const currency = values[1];
      const expected_currency_amount = values[2];
      const toAddress = make_sell_address(currency, expected_currency_amount);
      inputAddressList.push(toAddress);
      outputAddressList.push(toAddress);
      payload = action+","+values[0]+","+currency+","+expected_currency_amount;

    }	

    var enc = new TextEncoder('utf8');
    const payloadBytes = enc.encode(payload);
    const transactionHeaderBytes = protobuf.TransactionHeader.encode({
      familyName: FAMILY_NAME,
      familyVersion: FAMILY_VER,
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
    return fetch('http://rest-api:8008/batches', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/octet-stream'
      },
      body: batchListBytes
    })
    .then((response) => response.json())
    .then((responseJson) => {
      console.log(responseJson);
      return responseJson;
    })
    .catch((error) => {
      console.error(error);
    }); 	
  }
}
module.exports.dgcRequest = dgcRequest;
