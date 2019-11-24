// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

'use strict'

const _ = require('lodash')
const express = require('express');
const bodyParser = require('body-parser');
const {dgcWalletRequest} = require('./dgcWalletRequest') 
const urlencodedParser = bodyParser.urlencoded({ extended: false })
const router = express.Router();

// makePrivateKey
router.post('/makePrivateKey', function(req, res){
    let app = new dgcWalletRequest(req.body.privateKey);
    res.send({privateKey: app.makePrivateKey()});
})

// getPublicKey
router.post('/getPublicKey', function(req, res){
    let app = new dgcWalletRequest(req.body.privateKey);
    var isPrivateKey = app.isPrivateKey();
    if (app.isPrivateKey()) {
        res.send({publicKey: app.getPublicKey()});
    }
    if (null == req.body.privateKey) {
        res.send({error: "privateKey is empty"});
    } else {
        //var client = new dgcWalletRequest(req.body.privateKey);
        res.send({publicKey: client.getPublicKey()});
    }
})

// Is Registered
router.get('/isRegistered', function(req, res){
    if (null == req.body.privateKey) {
        res.send({message: 'You have not the privateKey'})
    }
})

// dgcBalance
router.post('/dgcBalance', function(req, res){
    if (null == req.body.privateKey) {
        res.send({error: "privateKey is empty"});
    } else {
        var client = new dgcWalletRequest(req.body.privateKey);
        var getBalance = client.dgcBalance();
        getBalance.then(result => {
            res.send({ balance: result, message:"Amount " + result + " available"});
        });
    }
})

// Transfer DGC to another user
router.post('/dgcTransfer', function(req, res) {
    if (null == req.body.privateKey) {
        res.send({error: "privateKey is empty"});
    } else if (null == req.body.beneficiary) {
        res.send({error: "beneficiary is empty"});
    } else {
        var client = new dgcWalletRequest(req.body.privateKey);
        var getBalance = client.dgcBalance();
        getBalance.then(result => {
            if (req.body.DGC > result ) {
                res.send({ balance: result, message:"your DGC balance is not enough"});
            } else {
                var beneficiary = req.body.beneficiary;
                var amount = req.body.DGC;
                client.dgcTransfer(beneficiary, amount);
                res.send({ message:"Amount "+ amount +" successfully added to " + beneficiary});        
            }
        });
    }
});

// Buy DGC from marketplace
router.post('/buyDGC', function(req, res) {
    if (null == req.body.privateKey) {
        res.send({error: "privateKey is empty"});
    } else if (null == req.body.currency) {
        res.send({error: "Currency cannot be empty"});
    } else {
        var client = new dgcWalletRequest(req.body.privateKey);
        var getBalance = client.dgcBalance();
        getBalance.then(result => {
            if (req.body.DGC > result ) {
                res.send({ balance: result, message:"your DGC balance is not enough"});
            } else {
                var beneficiary = req.body.beneficiary;
                var amount = req.body.DGC;
                client.dgcTransfer(beneficiary, amount);
                res.send({ message:"Amount "+ amount +" successfully added to " + beneficiary});        
            }
        });
    }
});

// sell DGC to marketplace
router.post('/sellDGC', function(req, res) {
    if (null == req.body.privateKey) {
        res.send({error: "privateKey is empty"});
    } else if (null == req.body.currency) {
        res.send({error: "Currency cannot be empty"});
    } else {
        var client = new dgcWalletRequest(req.body.privateKey);
        var getBalance = client.dgcBalance();
        getBalance.then(result => {
            if (req.body.DGC > result ) {
                res.send({ balance: result, message:"your DGC balance is not enough"});
            } else {
                var sellAmount = req.body.DGC;
                let buyingList = client.dgcBuyingList();
                for (i = 0; i < buyingList.length; i++) {
                    if (sellAmount > buyingList[i].DGC) {
                        var beneficiary = buyingList[i].beneficiary;
                        client.buyDGC(beneficiary, buyingList[i].DGC);
                        sellAmount = sellAmount - buyingList[i].DGC;
                    } else {

                    }
                }
                res.send({ message:"Amount "+ amount +" successfully added to " + beneficiary});        
            }
        });
    }
});

// Get Info
router.get('/info', function(req, res){
    var userId = req.body.userId;
    res.send({
        //pubkey: batcher.getPublicKey(),
        //mapsApiKey: config.MAPS_API_KEY,
        endpoints: endpointInfo
    });
})

// Parses the endpoints from an Express router
const getEndpoints = router => {
    return _.chain(router.stack)
    .filter(layer => layer.route)
    .map(({ route }) => {
        return _.chain(route.stack)
        .reduceRight((layers, layer) => {
            if (layer.name === 'restrict') {
                _.nth(layers, -1).restricted = true
            } else {
                layers.push({
                    path: route.path,
                    method: layer.method.toUpperCase(),
                    restricted: false
                })
            }
            return layers
        }, [])
        .reverse()
        .value()
    })
    .flatten()
    .value()
}
const endpointInfo = getEndpoints(router)

// Copyright (c) The dgc.network
router.get('/', function(req, res){
    //res.redirect("/login");
    res.render('homePage');
})

//Get home view
router.get('/login', function(req, res){
    res.render('loginPage');
});

//Get main view
router.get('/home', function(req, res){
    res.render('homePage');
});

// Get Deposit view
router.get('/deposit',function(req, res){
    res.render('depositPage');
})

//Get Withdraw view
router.get('/withdraw',function(req, res){
    res.render('withdrawPage');
})

//Get Transfer View
router.get('/transfer',function(req, res){
    res.render('transferPage');
})

//Get Balance View
router.get('/balance', function(req, res){
    res.render('balancePage');
})

//recieve data from login page and save it.
router.post('/login', urlencodedParser, function(req, res){
    var userid = req.body.userId;
    res.send({done:1, userId: userid, message: "User Successfully Logged in as "+userid  });
});

//function to deposit amount in server
router.post('/deposit', function(req, res) {
    //var userId = req.body.userId;
    //var client = new dgcWalletRequest(userId); 
    var amount = req.body.money;
    var privateKey = req.body.privateKey;
    var client = new dgcWalletRequest(privateKey);
    client.deposit(amount);    
    res.send({message:"Amount "+ amount +" successfully added"});
});

//function to withdraw
router.post('/withdraw', function(req, res) {
    //var userId = req.body.userId;
    //var client = new dgcWalletRequest(userId);   
    var amount = req.body.money;
    var privateKey = req.body.privateKey;
    var client = new dgcWalletRequest(privateKey);
    client.withdraw(amount);     
    res.send({  message:"Amount "+ amount +" successfully deducted"});
});

//function to transfer money to another user
router.post('/transfer', function(req, res) {
    //var userId = req.body.userId;
    //var client = new dgcWalletRequest(userId);
    var beneficiary = req.body.beneficiary;
    var amount = req.body.money;
    var privateKey = req.body.privateKey;
    var client = new dgcWalletRequest(privateKey);
    client.transfer(beneficiary, amount);    
    res.send({ message:"Amount "+ amount +" successfully added to " + beneficiary});
});

router.post('/balance', function(req, res){
    //var userId = req.body.userId;
    //var client = new dgcWalletRequest(userId);
    var privateKey = req.body.privateKey;
    var client = new dgcWalletRequest(privateKey);
    var getYourBalance = client.balance();
    console.log(getYourBalance);
    getYourBalance.then(result => {res.send({ balance: result, message:"Amount " + result + " available"});});
})

module.exports = router;
