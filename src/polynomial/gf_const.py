gf_exp = [1] * 512
gf_log = [0] * 256
x = 1
for i in range(1,255):
  x <<= 1
  if x & 0x100:
      x ^= 0x11d
  gf_exp[i] = x
  gf_log[x] = i
for i in range(255,512):
  gf_exp[i] = gf_exp[i-255]

print "const GF_EXP: [u8; 512] = %s" % gf_exp
print "const GF_LOG: [u8; 256] = %s" % gf_log
