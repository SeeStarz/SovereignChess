# Does not account for changed header files

SHELL = cmd
VPATH = ..\\src
SFML = ..\\..\\libs\\SFML-2.6.1-manual
FLAGS = -fdiagnostics-color=always -g -pthread -Wall -Wextra -Wpedantic
objects = boardmanager.o button.o config.o gamestate.o helper.o loadable.o main.o mainmenu.o move.o piece.o

main : $(objects)
	g++ $(FLAGS) \
	$(objects) -o ..\\dll\\main \
	-L$(SFML)\\lib \
	-lsfml-graphics -lsfml-window -lsfml-system -lsfml-network -lsfml-audio

%.o : %.cpp
	g++ $(FLAGS) \
	-I$(SFML)\\include \
	$< -o $@ -c

.PHONY : clean
clean :
	del *.o
	del ..\\dll\\main.exe