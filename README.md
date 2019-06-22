Sighting DB is designed to scale writing and reading a count of attributes, tracking when if was first and last seen.

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

