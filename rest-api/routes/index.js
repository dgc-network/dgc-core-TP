// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

'use strict'

const _ = require('lodash')
const express = require('express');
const bodyParser = require('body-parser');
const {dgcRequest} = require('./dgcRequest') 
const urlencodedParser = bodyParser.urlencoded({ extended: false })
const router = express.Router();

// makePrivateKey
router.post('/makePrivateKey', function(req, res){
    let app = new dgcRequest(req.body.privateKey);
    app.isPrivateKey().then(result => {
        if (false == result) {
            res.send({ privateKey: app.makePrivateKey()});
        } else {
            res.send({ error: "privateKey is existed"});
        }
    });
})

// getPublicKey
router.post('/getPublicKey', function(req, res){
    let app = new dgcRequest(req.body.privateKey);
    app.isPrivateKey().then(result => {
        if (false == result) {
            res.send({ error: "privateKey is not corrected"});
        } else {
            res.send({ publicKey: app.getPublicKey()});
        }
    });
})

// dgcBalance
router.post('/dgcBalance', function(req, res){
    let app = new dgcRequest(req.body.privateKey);
    app.dgcBalance().then(result => {
        if (false == result) {
            res.send({ error: "privateKey is not corrected"});
        } else {
            res.send({ balance: result, message:"Amount " + result + " available"});
        }
    });
})

// dgcCredit
router.post('/dgcCredit', function(req, res){
    let app = new dgcRequest(req.body.privateKey);
    app.dgcBalance().then(result => {
        if (false == result) {
            res.send({ error: "privateKey is not corrected"});
        } else {
            res.send({ credit: result, message:"Amount " + result + " available"});
        }
    });
})

// Transfer DGC to another user
router.post('/transferDGC', function(req, res) {
    let app = new dgcRequest(req.body.privateKey);
    app.dgcBalance().then(result => {
        if (false == result) {
            res.send({ error: "privateKey is not corrected"});
        } else {
            if (req.body.DGC > result ) {
                res.send({ message: "your DGC balance is not enough"});
            } else {
                var amount = req.body.DGC;
                var beneficiary = req.body.beneficiary;
                app.dgcTransfer(amount, beneficiary);
                res.send({ message:"Amount "+ amount +" successfully added to " + beneficiary});        
            }
        }
    });
});

// dgcExchange
router.post('/dgcExchange', function(req, res){
    let currency = req.body.currency;
    let app = new dgcRequest(req.body.privateKey);
    app.dgcExchange(currency).then(result => {
        if (false == result) {
            res.send({ message:"the currency " + currency + " is not existed"});
        } else {
            res.send({ exchange: result, message:"The currency "  + currency + " exchange rate is " + result });
        }
    });
})

// sell DGC to marketplace
// imcomplete
router.post('/sellDGC', function(req, res) {
    if (null == req.body.privateKey) {
        res.send({error: "privateKey is empty"});
    } else if (null == req.body.currency) {
        res.send({error: "Currency cannot be empty"});
    } else {
        var app = new dgcRequest(req.body.privateKey);
        var getBalance = app.dgcBalance();
        getBalance.then(result => {
            if (req.body.DGC > result ) {
                res.send({ balance: result, message:"your DGC balance is not enough"});
            } else {
                var sellAmount = req.body.DGC;
                let buyingList = app.dgcBuyingList();
                for (i = 0; i < buyingList.length; i++) {
                    if (sellAmount > buyingList[i].DGC) {
                        var currency = buyingList[i].currency;
                        app.buyDGC(buyingList[i].DGC, currency);
                        sellAmount = sellAmount - buyingList[i].DGC;
                    } else {

                    }
                }
                res.send({ message:"Amount "+ amount +" successfully sell to " + currency});        
            }
        });
    }
});

// Buy DGC from marketplace
// imcomplete
router.post('/buyDGC', function(req, res) {
    let amount = req.body.DGC;
    let currency = req.body.currency;
    let currency_amount = req.body.currency_amount;
    let app = new dgcRequest(req.body.privateKey);
    app.buyDGC(amount, currency, currency_amount);
    
    
    
    res.send({ message:"Amount "+ amount +" successfully buy from " + currency});        
    
    app.dgcExchange(currency).then(result => {
        if (false == result ) {
            res.send({ message:"the currency " + currency + " is not existed"});
        } else {
            app.buyDGC(amount, currency, currency_amount);
            res.send({ message:"Amount "+ amount +" successfully buy from " + currency});        
        }
    });
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
    //var client = new dgcRequest(userId); 
    var amount = req.body.money;
    var privateKey = req.body.privateKey;
    var client = new dgcRequest(privateKey);
    client.deposit(amount);    
    res.send({message:"Amount "+ amount +" successfully added"});
});

//function to withdraw
router.post('/withdraw', function(req, res) {
    //var userId = req.body.userId;
    //var client = new dgcRequest(userId);   
    var amount = req.body.money;
    var privateKey = req.body.privateKey;
    var client = new dgcRequest(privateKey);
    client.withdraw(amount);     
    res.send({  message:"Amount "+ amount +" successfully deducted"});
});

//function to transfer money to another user
router.post('/transfer', function(req, res) {
    //var userId = req.body.userId;
    //var client = new dgcRequest(userId);
    var beneficiary = req.body.beneficiary;
    var amount = req.body.money;
    var privateKey = req.body.privateKey;
    var client = new dgcRequest(privateKey);
    client.transfer(beneficiary, amount);    
    res.send({ message:"Amount "+ amount +" successfully added to " + beneficiary});
});

router.post('/balance', function(req, res){
    //var userId = req.body.userId;
    //var client = new dgcRequest(userId);
    var privateKey = req.body.privateKey;
    var client = new dgcRequest(privateKey);
    var getYourBalance = client.balance();
    console.log(getYourBalance);
    getYourBalance.then(result => {res.send({ balance: result, message:"Amount " + result + " available"});});
})

module.exports = router;