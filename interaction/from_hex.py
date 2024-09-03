import sys

for i in range(1, len(sys.argv)):
    x = sys.argv[i]

    try:
        x = bytes.fromhex(x).decode('utf-8')
        if x.isdigit():
            x = '"' + x + '"'
    except:
        print(x)
        if x[0:2] != '0x':
            x = '0x' + x
        x = int(x, base=0)

    
    print(str(x))

