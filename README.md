# Legend Engine
The legend engine is a 2D text based game engine written in Rust

## Running a game for the Legend Engine
The Legend game info is stored in a `game` directory next to the binarie, if your have a valid [game manifest](https://github.com/Tau5/legend/wiki/game-manifest) just run the binarie (The binarie must be in the same directory as the `game` directory) and the game will run


## Creating games for the Legend Engine

To make games you can use the **Legend SDK**

### Instalation of the Legend SDK

First, install **NodeJS 8 LTS** or higher

When its installed, install the SDK with this command 

Note: It is recommended to use [nvm](https://github.com/creationix/nvm) if you dont want to give superuser privilegies to NPM to install
```
npm install --global legendsdk
```
this will install the NPM package of the Legend SDK

### Creating a game proyect

To create a game proyect, run this command:
```
legendsdk init
```
The SDK will ask some questions to configure the proyect and will create a `game` directory and the manifest with the provieded configuration

### Creating a map

Now that you created your proyect, let's create a map.

If you has followed the tutorial you should specified a intial map when you created the proyect, in the command bellow use the inital map filenme as the filename for the map

First we are gonna move to the new game directory, next run the following command to create a new map (In filename, specifie the filename)
```
legendsdk new <filename>
```
The Legend SDK will ask you a few questions, the `rows` and `columns` are the size of your map.

### Editing a map

To edit a map run the following command:
```
legendsdk edit <filename>
```
For an explanation of the editor read the [wiki page](https://github.com/Tau5/legend/wiki/editor)
