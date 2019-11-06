#!/usr/bin/python3
import json
import requests

jdata = json.dumps({"items": [{"namespace": "foo/bar/ip", "value":"127.0.0.1"}, {"namespace": "foo/bar/ip", "value":"192.168.0.12"}]})
jheaders={"content-type": "application/json"}

#print(str(data))

r = requests.post("https://127.0.0.1:9999/wb", headers=jheaders, data=jdata, verify=False)
print(r.status_code)
print(r.text)
