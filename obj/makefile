# Does not account for changed header files

SHELL = cmd
VPATH = ..\\src
SFML = ..\\..\\libs\\SFML-2.6.1-manual
objects = boardmanager.o button.o config.o gamestate.o helper.o loadable.o main.o mainmenu.o move.o piece.o

main : $(objects)
	g++ -fdiagnostics-color=always -g -pthread \
	$(objects) -o ..\\dll\\main \
	-L$(SFML)\\lib \
	-lsfml-graphics -lsfml-window -lsfml-system -lsfml-network -lsfml-audio

%.o : %.cpp
	g++ -fdiagnostics-color=always -g -pthread \
	-I$(SFML)\\include \
	$< -o $@ -c

.PHONY : clean
clean :
	del *.o
	del ..\\dll\\main.exe