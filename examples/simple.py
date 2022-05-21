from pow_py import PoWConfig, Work

c = PoWConfig("foobar")
p = c.work("da498347e52b958502dce3ff5f8b06d6bfd890a7ecef2dec93525c778959", 50000)
print(p.result)
print(p.nonce)
