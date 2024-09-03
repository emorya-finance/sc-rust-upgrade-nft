import sys

for i in range(1, len(sys.argv)):
    x = sys.argv[i]
    
    if x.isdigit():
        x = hex(int(x))
        if len(x) % 2 == 1:
            x = x.replace('0x', '0x0')
    else:
        x = x.encode('utf-8').hex()
    
    print(x.replace('0x', ''))