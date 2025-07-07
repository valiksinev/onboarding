1. install solana-tools on local machine:
	RUN sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.7/install)" && \
    /root/.local/share/solana/install/active_release/bin/sdk/sbf/scripts/install.sh

(full description: https://solana.com/docs/intro/installation)
	
test:
	$ solana --version
	solana-cli 2.1.21 (src:8a085eeb; feat:1416569292, client:Agave)

2. Install docker
	https://docs.docker.com/engine/install/ubuntu/

test:
	$ sudo docker run hello-world

	Unable to find image 'hello-world:latest' locally
	latest: Pulling from library/hello-world
	e6590344b1a5: Pull complete 
	Digest: sha256:940c619fbd418f9b2b1b63e25d8861f9cc1b46e3fc8b018ccfe8b78f19b8cc4f
	Status: Downloaded newer image for hello-world:latest

	Hello from Docker!
	This message shows that your installation appears to be working correctly.

3. Install docker-compose
	https://docs.docker.com/compose/install/standalone/

test: 
	$ docker-compose --version
	Docker Compose version v2.29.2

4. clone repo
    using ssh (recommended): git@github.com:valiksinev/onboarding.git
    using https: https://github.com/valiksinev/onboarding

It is recommended to use ssh key to work with github.
If your github account doesn't have ssh key, you can use this link to add ssh-key:
    https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account


5. start solana-validator
    $ cd onboarding/step_1
    $ docker-compose up solana

6. create keypair
    $ solana-keygen
    just press "enter" to skip "BIP39 Passphrase"

test:
    $ solana address
    you can see  public key of your keypair

    to see the location of your keypair run solana-cli:
    $ solana config get

7. airdrop funds to your keypair
    $ solana airdrop 100

test
    $ solana balance
    100 SOL

8. try to understand something about solana transaction from doc:
    https://solana.com/docs/core/transactions

8. clone repo of the spl-memo program
    git@github.com:solana-program/memo.git

9. create solana-transaction for spl-memo contract.
    Memo folder contains cli-memo program that creates and sends transaction for "memo" contract.

    Memo contract  - a pretty simple contract that validates a string of UTF-8 encoded characters and
    logs it in the transaction log (you can read about in readme: https://github.com/solana-program/memo/tree/main)

    You need to investigate the source of the memo-cli. This example creates solana transaction, send it to validator,
    waits for confirmation of the transaction execution.

run memo-cli:
    $ cd memo-cli
    $ cargo run

result in solana logs:
    solana-1  | [2025-07-07T09:48:33.575651354Z DEBUG solana_runtime::message_processor::stable_log] Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr invoke [1]
    solana-1  | [2025-07-07T09:48:33.575962755Z DEBUG solana_runtime::message_processor::stable_log] Program log: Signed by AwsrRzgubeCsP77GNRdGCH4XefWkczFPrAzm86q7gVv4
    solana-1  | [2025-07-07T09:48:33.576036717Z DEBUG solana_runtime::message_processor::stable_log] Program log: Memo (len 11): "hello world"
    solana-1  | [2025-07-07T09:48:33.576346171Z DEBUG solana_runtime::message_processor::stable_log] Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr consumed 19096 of 200000 compute units
    solana-1  | [2025-07-07T09:48:33.576358784Z DEBUG solana_runtime::message_processor::stable_log] Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr success
    solana-1  | [2025-07-07T09:48:33.583999098Z DEBUG solana_runtime::message_processor::stable_log] Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr invoke [1]
    solana-1  | [2025-07-07T09:48:33.584284323Z DEBUG solana_runtime::message_processor::stable_log] Program log: Signed by AwsrRzgubeCsP77GNRdGCH4XefWkczFPrAzm86q7gVv4
    solana-1  | [2025-07-07T09:48:33.584366828Z DEBUG solana_runtime::message_processor::stable_log] Program log: Memo (len 11): "hello world"
    solana-1  | [2025-07-07T09:48:33.584731743Z DEBUG solana_runtime::message_processor::stable_log] Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr consumed 19096 of 200000 compute units
    solana-1  | [2025-07-07T09:48:33.584744450Z DEBUG solana_runtime::message_processor::stable_log] Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr success

10. use solscan
    open https://solscan.io
    (in the upper right corner ) set Custom RPC: http://localhost:8899
    set tx signature to "search", see transaction logs

11. use solana-cli to fetch logs:
    $ solana transaction-history AwsrRzgubeCsP77GNRdGCH4XefWkczFPrAzm86q7gVv4 --show-transactions

   (replace AwsrRzgubeCsP77GNRdGCH4XefWkczFPrAzm86q7gVv4 by public key of your keypair)

12. Optionally: eExtend memo-cli program to fetch transaction logs using client.get_transaction() method