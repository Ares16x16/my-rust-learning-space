import socket
import time

HOST = '127.0.0.1'
PORT = 8000

while (1):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        print("Connected\n")
        s.sendall(b'Hello, world')
        data = s.recv(1024)
        print('Received', repr(data))
        time.sleep(1)

