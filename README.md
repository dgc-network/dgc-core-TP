# Introduction

This is a minimal example of a sawtooth 1.0 application. This example demonstrates, a common usecase, where a customer deposits/withdraws/transfers money from a wallet account.

A customer can:
1. deposit money into his/her wallet account
2. withdraw money from his/her wallet account
3. check the balance in the wallet account
4. transfer money from his/her wallet account to another

The customer is identified by a customer name and a corresponding public key. The wallet account balance, is stored at an address, derived from SHA 512 hash of customer's public key and the dgc-wallet transaction family namespace.

# Components
The application is built in two parts:
1. The client application written in Python, written in two parts: _client.py file representing the backend stuff and the _cli.py representing the frontend stuff. The example is built by using the setup.py file located in one directory level up.

2. The Transaction Processor is written in C++11 using c++-sawtooth-sdk. It comes with its CMake files for build. The Transaction Processor is also available in Java and Python.

------

**JAVASCRIPT CLIENT NOTE**

The client is also written in Javascript using node.js. The `app.js` is the main javascript file from where the `main` function call occurs. Handlebars are used for templating, client related CSS and JavaScript code is written in public folder and server related files are written in `router/` folder. Running the default `docker-compose.yaml` file or the `dgc-wallet-build-client-js.yaml` launches the client, which is accessible at `localhost:3000`. 

How to use the dgc-wallet UI:

1. Build and start the Docker containers:

`docker-compose -f dgc-wallet-build-client-js.yaml up`

2. Open bash shell in `dgc-wallet-client-js` container:

`docker exec -it dgc-wallet-client-js bash`

3. Create user accounts for jack and jill:

`sawtooth keygen jack && sawtooth keygen jill`

4. Open two new browser tabs and go to `http://localhost:3000` on each tab

5. Login in one tab as `jack` and in other as `jill` 

6. Start with an initial deposit for each user - jack and jill via the `Deposit` tab in the UI homepage

------

# Pre-requisites

This example uses docker-compose and Docker containers. If you do not have these installed please follow the instructions here: https://docs.docker.com/install/

**NOTE:**
The preferred OS environment is Ubuntu 16.04.3 LTS x64. Although, other linux distributions which support Docker should work. 
If you have Windows please install [Docker Toolbox for Windows](https://docs.docker.com/toolbox/toolbox_install_windows/) or [Docker for Windows](https://docs.docker.com/docker-for-windows/), based on your OS version.

**NOTE:**
The minimum version of Docker Engine necessary is 17.03.0-ce. Linux distributions often ship with older versions of Docker.

[Here's a gist](https://gist.github.com/dgc-network/76e348e34d93fc22926d7d9379a0fd08) detailing steps on installing docker and docker-compose.

### Working with proxies

**For linux:**

Follow the instructions in [sawtooth-core/BUILD.md](https://github.com/hyperledger/sawtooth-core/blob/master/BUILD.md#step-two-configure-proxy-optional)

**For pre-Windows 10 versions** (using Docker Toolbox for Windows):

Start the virtualbox host with:
```
   docker-machine rm default
   docker-machine create -d virtualbox --engine-env HTTP_PROXY=<your_http_proxy> --engine-env HTTPS_PROXY=<your_https_proxy> --engine-env NO_PROXY=<your_no_proxy> default
```   
When you start Kitematic it will initially show an error, but just click "Use Virtualbox". 

**For Windows 10** (using Docker for Windows):

Right click on the Docker icon in the notification area, select Settings. Then click Proxies. Select "Manual proxy configuration" and enter the following then click Apply.
```
    Web Server (HTTP):         <your_http_proxy>
    Secure Web Server (HTTPS): <your_https_proxy>
    Bypass for these hosts:    <your_no_proxy>,localhost,127.0.0.1
```

# Usage

Start the pre-built Docker containers in docker-compose.yaml file, located in dgc-wallet-TP directory:
```bash
cd dgc-wallet-tp
docker-compose up
```
At this point all the containers should be running.

To launch the client, you could do this:
```bash
docker exec -it dgc-wallet-client bash
```

You can locate the right Docker client container name using `docker ps`.

Sample command usage:

```bash
#Create a wallet
sawtooth keygen jack #This creates the public/private keys for Jack, a pre-requisite for the following commands

dgc-wallet deposit 100 jack #This adds 100 units to Jack's account

dgc-wallet withdraw 50 jack #Withdraws 50 units from Jack's account

dgc-wallet balance jack #Displays the balance left in Jack's account

#Create 2nd wallet
sawtooth keygen jill #This creates the public/private keys for Jill, a pre-requisite for the following commands

dgc-wallet deposit 100 jill #This adds 100 units to Jill's account

dgc-wallet balance jill #Displays the balance left in Jill's account

dgc-wallet transfer 25 jack jill #Transfer 25 units money from Jack to Jill

dgc-wallet balance jack #Displays the balance left in Jack's account

dgc-wallet balance jill #Displays the balance left in Jill's account

```

# Building containers
To build TP code of your preferred language and run the dgc-wallet example:

```bash
docker-compose -f dgc-wallet-build-tp-<your_prog_language>.yaml up --build
```
where,
 <your_prog_language> should be replaced with either `cxx`, `java`, or `py`

# Building and running on OS(without dockers)
To run dgc-wallet-TP without dockers, we'll have to use a Ubuntu 16.04 OS installation and compile dgc-wallet from sources. Below is a sample procedure for python TP/client:

1. Install sawtooth on Ubuntu 16.04 LTS x64 machine and setup genesis block. Refer sawtooth app developer's guide [here](https://sawtooth.hyperledger.org/docs/core/releases/latest/app_developers_guide/ubuntu.html)
   - Start the validator, rest-api and settings-tp in separate, new consoles:

     ```bash
     sudo -u sawtooth sawtooth-validator -vv
     sudo -u sawtooth sawtooth-rest-api -vvv
     sudo -u sawtooth settings-tp -vv
     ```
2. In a new console, clone the dgc-wallet repo:

   `git clone https://github.com/dgc-network/dgc-wallet-tp.git`
3. `cd dgc-wallet-tp`
4. Modify two files:
   - Create a new branch to start making changes
   `git checkout -b nodocker`
   - Edit file `pyclient/wallet/dgc_wallet_cli.py` and change `rest-api:8008` to `localhost:8008`
   - Edit file `pyprocessor/processor/dgc_wallet_tp.py` and change `validator:4004` to `localhost:4004`
5. Setup the dgc-wallet-tp.
   - Follow the [`pyprocessor/Dockerfile`](https://github.com/dgc-network/dgc-wallet-tp/blob/master/pyprocessor/Dockerfile)
   - Install all the dependencies in the [first `RUN` line](https://github.com/dgc-network/dgc-wallet-tp/blob/master/pyprocessor/Dockerfile#L18) in the Dockerfile
   - Run the dgc-wallet-tp with `./pyprocessor/dgc-wallet-tp`
6. Setup the client. Open a new console.
   - Follow the [`pyclient/Dockerfile`](https://github.com/dgc-network/dgc-wallet-tp/blob/master/pyclient/Dockerfile)
   - Install all the dependencies in the [first `RUN` line](https://github.com/dgc-network/dgc-wallet-tp/blob/master/pyclient/Dockerfile#L20) in the pyclient/Dockerfile
   - Run the dgc-wallet client with `./pyclient/dgc-wallet` command. Refer [Usage](#Usage) section above, for examples.

**NOTE** If you prefer using the dgc-wallet client without directory prefix, you could add the `pyclient` directory to your `PATH` environment variable, as shown below:

`export PATH = $PATH:<absolute-path-to-your-cloned-dgc-wallet-TP-dir>/pyclient"`

# Video Presentation
A video presentation on YouTube uses this application as an example. It is "Hyperledger Sawtooth Application Development using 'dgcWallet'". It is at https://youtu.be/4VUaZkwJdCc

# Contributing
Currently, we're looking for contributions and PRs for following:
 - TP using .NET sawtooth SDK
 - Client apps in GO, .NET, C++, and Java

More ways you can contribute:
 - Bugs or issues: Report problems or defects found when working with dgc-wallet
 - Core features and enhancements: Provide expanded capabilities or optimizations
 - Documentation: Improve existing documentation or create new information
 - Tests: Add functional, performance, or scalability tests

# License
This example and Hyperledger Sawtooth software are licensed under the [Apache License Version 2.0](LICENSE) software license.
# dgc-wallet-tp
