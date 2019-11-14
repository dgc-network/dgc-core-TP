// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

const _ = require('lodash')
var express = require('express');
var bodyParser = require('body-parser');
var router = express.Router();
var {dgcWalletRequest} = require('./dgcWalletRequest') 
const auth = require('./auth')

var urlencodedParser = bodyParser.urlencoded({ extended: false })

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

//makePrivateKey
router.post('/makePrivateKey', function(req, res){
    let response = auth.makePrivateKey;
    console.log(response);
    res.send({ privateKey:response.privateKey, publicKey:response.publicKey});
})

//Is Registered
router.get('/isregistered', function(req, res){
    var userId = req.body.userId;
    res.send({
        //pubkey: batcher.getPublicKey(),
        //mapsApiKey: config.MAPS_API_KEY,
        endpoints: endpointInfo
    });
})

//Get Info
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

module.exports = router;
