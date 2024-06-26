# Sovereign Chess

This project is a digital app version of a chess variation "Sovereign Chess" by Mark Bates. This project is a hobby project, expect there to be bugs and messy undocumented code.  

### ~~**!!! IMPORTANT !!! Currently only supports windows OS. Code is tested with windows 11.**~~
Now supports both windows and linux via flatpak

This project is by no means official, and this variation is not mine. Have a look at https://www.infinitepigames.com/sovereign-chess.

## Features
-Base sovereign chess game with complete ruleset.  
-Bases are colored according to the player who own it.  
-Shows current main army color of each player.  
-Shows whose player turn it is.  
-Confirmation on second turn whether second player would like to play as black or white.  
~~-Game is currently only in singleplayer with no AI. Bring a friend over or stream your game for all to see.~~  
-No AI but you can now play multiplayer!

## How To Play
-Rules: https://www.infinitepigames.com/sc-rules
-Just click and click, mostly self explanatory  
-Pawn promotion to a king will be available if it is safe  
-To castle click the king and then click the king destination  
-To castle one square click the king and then click the rook  
-To do defection click on colored kings on the bottom right, then select a valid square (usually the king itself)  

## Multiplayer
-Input a valid port (they probably should be the same on both machine idk don't ask).  
-If you are hosting just leave the host address empty (idk what it does, really).  
-If you are connecting, write down the host ip address.  
-On local network (same wifi) you can "ipconfig" in cmd and copy paste the ipv4 address.  
-On remote network, you can use hamachi's ip address.  
-Port forwarding probably works, try yourself idk how to do it.

## Credits
Variation maker: Mark Bates.  
https://www.infinitepigames.com/about  
Library: https://www.sfml-dev.org/ under Zlib.  
https://www.sfml-dev.org/license.php  
Pieces: https://github.com/lichess-org/lila/tree/master/public/piece/pixel under AGPLv3+.  
-Changes to pieces: converted svg to png and ico, changed color to be monochromatic, modify pngs for piece bases  
https://www.gnu.org/licenses/agpl-3.0.en.html  
Audio: https://freesound.org/people/el_boss/packs/30764/ under CC0.  
https://creativecommons.org/publicdomain/zero/1.0/legalcode.en

## Images
<img src="https://i.imgur.com/7A515Zp.png" width="600" />
<img src="https://i.imgur.com/Zrtr3cx.png" width="600" />
<img src="https://i.imgur.com/EugWGji.png" width="600" />

## Build This Project Yourself
### Version 1.4.x
####Windows
Download SFML 2.6.1 with compatible compiler/IDE  
https://www.sfml-dev.org/tutorials/2.6/  
Download MakeFile  
Locate MakeFile inside obj directory  
Run ```make -f makewindows``` or its alternative version (e.g. ming32-make)  
Executable can be found inside dll folder  
Run app using start.bat

####Linux
Install SFML 2.6.1 from your package manager or build it yourself
https://www.sfml-dev.org/tutorials/2.6/compile-with-cmake.php  
Arch: ```sudo pacman -S sfml```  
  
```git clone https://github.com/SeeStarz/SovereignChess.git```  
```cd SovereignChess```  
```make -C obj -f makelinux```  
Executable can be found inside dll folder
Run app using start.sh

### Version 1.2.x and 1.3.0  
Download SFML 2.6.1 with compatible compiler/IDE  
https://www.sfml-dev.org/tutorials/2.6/  
Download MakeFile  
Locate MakeFile inside obj directory  
Run ```make``` or its alternative version (e.g. ming32-make)  
Executable can be found inside dll folder  

### Version 1.1.x and 1.0.x
Download SFML 2.5.1 with compatible compiler/IDE  
https://www.sfml-dev.org/tutorials/2.5/  
compiler settings can be found in .vscode/tasks.json  
