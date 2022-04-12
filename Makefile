build-pyinstaller:
	pyinstaller --clean -y --onefile src/dotm.py

init:
	pip install -r requirements.txt

install: build-pyinstaller
	sudo cp dist/dotm /usr/bin/

clean:
	@echo "Removing build files..."
	@sudo rm -rf dist/ build/ dotm.spec