build-pyinstaller:
	pyinstaller --clean -y --onefile dotm.py

install: build-pyinstaller
	sudo cp dist/dotm /usr/bin/