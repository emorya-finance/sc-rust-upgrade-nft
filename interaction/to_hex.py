import sys
import subprocess

for i in range(1, len(sys.argv)):
    x = sys.argv[i]
    
    if x.startswith('erd') and len(x) == 62:
        result = subprocess.run(['mxpy', 'wallet', 'bech32', '--decode', x], capture_output=True, text=True)
        x = result.stdout.strip()
    elif x.isdigit():
        x = hex(int(x))
        if len(x) % 2 == 1:
            x = x.replace('0x', '0x0')
    else:
        x = x.encode('utf-8').hex()
    
    print(x.replace('0x', ''))