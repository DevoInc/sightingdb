Sighting DB is designed to scale writing and reading a count of attributes, tracking when if was first and last seen.

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

The b64 command is just formating the string as base64 URL with no padding, it is build as part of this program, or you can make your own.

Writing
-------
	$ curl -k https://localhost:9999/w/my/namespace/?val=$(b64 127.0.0.1)
	{"message":"ok"}
	
	$ curl -k https://localhost:9999/w/another/namespace/?val=$(b64 127.0.0.1)
	{"message":"ok"}
	$ curl -k https://localhost:9999/w/another/namespace/?val=$(b64 127.0.0.1)
	{"message":"ok"}

Reading
-------
	$ curl -k https://localhost:9999/r/my/namespace/?val=$(b64 127.0.0.1)
	{"value":"127.0.0.1","first_seen":1566624658,"last_seen":1566624658,"source":"unknown","source_timestamp":1566624658,"count":1}
	
	$ curl -k https://localhost:9999/w/another/namespace/?val=$(b64 127.0.0.1)
	{"value":"127.0.0.1","first_seen":1566624686,"last_seen":1566624689,"source":"unknown","source_timestamp":1566624686,"count":2}
	

Storage
=======

Default storage is a simple tree which contains the path + value. The path is usually the type of value, such as a host, an IP address, a hash etc.

Read and Write on Multiple instances
====================================

We do a XOR on the available machines with the first four bytes of our value in order to avoid using a consensus to read and write our data. This
allow a blazingly fast access to our values.

Adding a new Node
=================

This is a slow operation which requires to re-XOR our values in order to move the appropriate ones to the new node. We take only one thread for this
so the DB is still fast, new writes go directly to the new node, while read is first fetch to the new node and then looked on the original node handling
them if the data has not been moved.

