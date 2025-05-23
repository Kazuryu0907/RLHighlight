import socket

M_SIZE = 1024

addr = ("localhost",12345)
sock = socket.socket(socket.AF_INET,socket.SOCK_DGRAM)

# sock.sendto("Hello!".encode("utf-8"), addr)
# sock.sendto('{"cmd":"scored"}'.encode("utf-8"), addr)
sock.sendto('{"cmd":"dbg"}'.encode("utf-8"), addr)