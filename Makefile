DIST=/bin

help: 
	@echo -e "install-cli \t\tinstall dotm cli"
	@echo -e "install-gui \t\tinstall dotm gui"
	@echo
	@echo -e "uninstall-cli	\tonly uninstall dotm cli"
	@echo -e "uninstall-gui \t\tonly uninstall dotm gui"
	@echo
	@echo -e "install	\t\tinstall both cli and gui"
	@echo -e "uninstall	\tuninstall both cli and gui"

install-cli: ./cli
	@echo "############# Installing dotm (cli) #############"
	@cd cli/; cargo build -r 
	cp cli/target/release/dotm ${DIST}/

install-gui: ./gui
	@echo "############# Installing dotm-gui #############"
	@cd gui/; cargo build -r 
	cp gui/target/release/dotm-gui ${DIST}/

uninstall-cli:
	@echo "############# Uninstalling dotm (cli) #################"
	rm -f ${DIST}/dotm

uninstall-gui: 
	@echo "############# Uninstalling dotm-gui ###############"
	rm -f ${DIST}/dotm-gui

install: install-cli install-gui
uninstall: uninstall-cli uninstall-gui
