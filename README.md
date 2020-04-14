<p align="center"><img src="doc/sightingdb-logo2_64.png"/></p>

SightingDB is a database designed for Sightings, a technique to count items. This is helpful for Threat Intelligence as Sightings allow
to enrich indicators or attributes with Observations, rather than Reputation.

Simply speaking, by pushing data to SightingDB, you will get the first time it was observed, the last time, its count.

However, it will also provide the following features:
* Keep track of how many times something was searched
* Keep track of the hourly statistics per item
* Get the consensus for each item (how many times the same value exists in another namespace)

SightingDB is designed to scale writing and reading.

Building
========

1) Make sure you have Rust and Cargo installed
2) Run ''make''

Running
=======

To run from the source directory:

1. Generate a certificate: `cd etc; mkdir ssl; cd ssl; openssl req -new -newkey rsa:2048 -days 365 -nodes -x509 -keyout key.pem -out cert.pem; cd ../..`
2. `ln -s etc/ssl ssl`
3. `ln -s etc/sighting-daemon.ini sighting-daemon.ini`
4. Start the Sighting DB: ./target/debug/sighting-daemon

Client Demo
===========

Writing
-------
	$ curl -k https://localhost:9999/w/my/namespace/?val=127.0.0.1
	{"message":"ok"}	
	$ curl -k https://localhost:9999/w/another/namespace/?val=127.0.0.1
	{"message":"ok"}
	$ curl -k https://localhost:9999/w/another/namespace/?val=127.0.0.1
	{"message":"ok"}

Reading
-------
	$ curl -k https://localhost:9999/r/my/namespace/?val=$(b64 127.0.0.1)
	{"value":"127.0.0.1","first_seen":1566624658,"last_seen":1566624658,"count":1,"tag":"","ttl":0,"consensus":2}
	
	$ curl -k https://localhost:9999/w/another/namespace/?val=127.0.0.1
	{"value":"127.0.0.1","first_seen":1566624686,"last_seen":1566624689,"count":2,"tag":"","ttl":0,"consensus":2}
	

