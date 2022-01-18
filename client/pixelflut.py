#!/usr/bin/python

import sys
import time
import socket
from PIL import Image

HOST = '127.0.0.1'
PORT = 1234

if len(sys.argv) != 2:
	print("Usage: python pixelflut.py <image>")
	exit()

with Image.open(sys.argv[1]) as im: 
	print(str(im.size))
	print(str(im.getpixel((0, 0))))

	while True:
		sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		try:
			sock.connect((HOST, PORT))
			for x in range(0, im.size[0]):
				for y in range(0, im.size[1]):
					(r, g, b, a) = im.getpixel((x, y))
					if a == 0:
						r = 0
						b = 0
						g = 0
					sock.send("PX {} {} {:02X}{:02X}{:02X}\n".format(x, y, r, g, b).encode())

		except Exception as e:
			print("Disconnected from server, retrying in 5 seconds")
			print(str(e))
			time.sleep(5)
