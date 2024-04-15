# Does not account for changed header files

SHELL = cmd
VPATH = ..\\src
SFML = ..\\..\\libs\\SFML-2.5.1
objects = boardmanager.o button.o gamestate.o helper.o loadable.o main.o mainmenu.o move.o piece.o

main : $(objects)
	g++ -fdiagnostics-color=always -g -pthread \
	$(objects) -o ..\\dll\\main \
	-L$(SFML)\\lib \
	-lsfml-graphics -lsfml-window -lsfml-system -lsfml-network

%.o : %.cpp
	g++ -fdiagnostics-color=always -g -pthread \
	-I$(SFML)\\include \
	$< -o $@ -c

.PHONY : clean
clean :
	del *.o
	del ..\\dll\\main.exe