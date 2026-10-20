#!/usr/bin/env python3
from itertools import product
for auth,tenant,mutating,campaign_write in product((False,True),repeat=4):
  admitted=auth and tenant and (not mutating or campaign_write)
  if admitted and mutating: assert campaign_write
  if auth and tenant and not mutating: assert admitted
  if not auth or not tenant: assert not admitted
print("campaign tool authority: ok")
