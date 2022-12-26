f = "nite{n4nd_1s_un1v3rs4l_g4te}"

news = b""

def bitswap(x, p1, p2):

    set1 =  (x >> p1) & 1

    set2 =  (x >> p2) & 1
  

    xor = (set1 ^ set2)
  


    xor = (xor << p1) | (xor << p2)
    result = x ^ xor
  
    return result

addd = 0
for i in f:
    val = (ord(i) ^ ((0xa3+addd)%256))
    val = bitswap(val,2,5)
    val = bitswap(val,7,3)
    print(hex(val),end=' ')
    news += bytes([val])
    addd += 3
print("")



addd = 0
news2 = b''
for i in news:
    val = bitswap(i,7,3)
    val = bitswap(val,2,5)
    
    val = ( val ^ ((0xa3+addd)%256))

    news2 += bytes([val])
    addd += 3
print(news2)


