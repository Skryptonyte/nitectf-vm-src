

def nand(r1, r2):
    return b'\x31' + bytes([(r1 << 4 )| r2])

def compare(r1, r2):
    return b'\x32' + bytes([(r1 << 4 )| r2])

def rightshift(r1,shift):
    return b'\x40' + bytes([(r1 << 4 )| shift])

def leftshift(r1,shift):
    return b'\x41' + bytes([(r1 << 4 )| shift])


def andgate(r1, r2):
    code =b''
    code += nand(r1,r2)
    code += nand(r1,r1)

    return code

def orgate(r1, r2):
    code =b''
    code += nand(r1,r1)
    code += nand(r2,r2)
    code += nand(r1,r2)
    return code

def add(r1, r2):
    return b'\x32' + bytes([(r1 << 4 )| r2])
def mov_reg(r1, r2):
    return b'\x30' + bytes([(r1 << 4 )| r2])


def mov_mem_to_reg(r1, loc):
    return bytes([(0x1 << 4 )| r1+8]) + bytes([ (loc&0xff00) >> 8, loc & 0xff])

def mov_reg_to_mem(r1, loc):
    return bytes([(0x2 << 4 )| r1]) + bytes([ (loc&0xff00) >> 8, loc & 0xff])

def mov_reg_to_mem(r1, loc):
    return bytes([(0x2 << 4 )| r1]) + bytes([ (loc&0xff00) >> 8, loc & 0xff])

def mov_imm_to_reg(r1, op):
    return bytes([(0x1 << 4 )| r1]) + bytes([op])


def compare_reg(r1, r2):
    pass

def xor():
    code = b''
    code += mov_reg(2,0)
    code += mov_reg(3,1) 
    code += nand(0,1)
    code += mov_reg(1,0)
    code += nand(2,0)
    code += nand(3,1)
    code += nand(2,3)

    return code

def bitswap(shift1, shift2):
    code = b''
    code += mov_reg(1,0)
    code += mov_reg(4,0)
    code += mov_imm_to_reg(5,1)
    code += rightshift(0,shift1)
    code += andgate(0,5)
    code += rightshift(1,shift2)
    code += andgate(1,5)
    code += xor()
    code += mov_reg(1,2)
    code += mov_reg(0,2)
    code += leftshift(0,shift1)
    code += leftshift(1,shift2)
    code += orgate(0,1)
    code += mov_reg(1,4)
    code += xor()
    return code
code = b''

code += mov_imm_to_reg(7,0)
code += mov_imm_to_reg(6,1)

code += b'\x81\xb0\x00'

index = 0xb000

def bitswap_test(x, p1, p2):

    set1 =  (x >> p1) & 1
    set2 =  (x >> p2) & 1
    xor = (set1 ^ set2)
    xor = (xor << p1) | (xor << p2)
    result = x ^ xor
  
    return result
def correctAns(i,val):
    val = (ord(i) ^ ((0xa3+val) % 256))
    val = bitswap_test(val,2,5)
    val = bitswap_test(val,7,3)

    return val

f = "nite{n4nd_1s_un1v3rs4l_g4te}"


correctMsgInd = 0
correctMessage = "You got it right!"
for c in correctMessage:
    code += mov_imm_to_reg(0,ord(c))
    code += mov_reg_to_mem(0,0xc000+correctMsgInd)
    correctMsgInd += 1

val = 0
for i in f:
    #code += mov_imm_to_reg(0,ord(i))
    code += mov_mem_to_reg(0,index)
    code += mov_imm_to_reg(1, (0xa3+val) % 256)
    code += xor()

    code += mov_reg(0,2)
    code += bitswap(2,5)
    code += mov_reg(0,2)
    code += bitswap(3,7)
    code += mov_reg_to_mem(2,index)
    
    print(hex(correctAns(i,val)))
    code += mov_imm_to_reg(1, correctAns(i,val))
    code += compare(1,2)
    code += b'\x50\x00\x00'
    val += 3
    index += 1


#code += b'\x80\xb0\x00'
code += b'\x80\xc0\x00'

open('encrypt','wb').write(code)

print(code)
print("Bytecode length:",len(code))