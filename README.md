# art2map
Minecraft setup (mod and server) for simple, comfortable and fast map-arts.

![art2map](https://user-images.githubusercontent.com/86967334/178962602-e6bcd322-453b-49f5-91b3-57e4076cb973.png)

## Why?
I was playing minecraft with my friend, and one day, we just thought of idea to create actual pictures on our bases.
So, I started searching for methods, but they were not that simple:
- a) Setup a server plugin (I just want to play with my friend. Why?);
- b) Use fabric mods (I want to play with forge mods);
- c) Build pictures myself (Long and boring);
- d) Convert every images using some site, and then paste in the *saves//world_name//data* directory (Still long, and has some issues with multiplayer)

So, I decided to make everything myself.
I dont know Java at all (And I dont know how to simply import **java.awt** package to my mod), so I made a server-method.

## How to use?
1. Install art2map.jar file to minecraft mods directory (Like normal);
2. Install art2map-server.exe on minecraft world host machine;
3. Just run the server. *(CTRL+C to stop it)*

Then, in your world, **while having empty map in your main hand**, you can just use:
`/art2map {url}`

As simple as it is.

But in case server cant start, and the reason is that port is already taken, you can change it by:
1. Restarting the server, and passing the custom port in the start.
2. By using `/art2map {port}` in your world, so mod will send requests to a new destination.

And as you get it, when your minecraft session ended - just stop the exe with CTRL+C.

## How it works?
As I said before, I was using [Mcreator](https://mcreator.net/) for the mod itself, and I couldnt find a single solution to install **java.awt** package for image processing. So my next thought was... Why dont I use a server to handle image processing?

So, the **art2map.jar** is a client. When you use **/art2map** with link in it:
1. minecraft server's host converts this link and makes a request to the local server. 
2. Local server then downloads the image, proccesses it (Resizing and color picking).
3. Local server sends a json byte array (This is a map [color array](https://minecraft.fandom.com/wiki/Map_item_format#map_.3C.23.3E.dat_format)) back to the client.
4. Java client receives this json array, converts it to a normal byte array.
5. Mod creates a new map item, fills its `colors` with received byte array, registers to a world.

And in the end, user receives the art he wanted on a map.


### Few details
*Server side can only process 1 task at a time. Why not use more workers? Because it will not make much sense. When server sends a request and waits for answer,
world freezes untill the task is done. People cant make more requests while world waits for array. Unless, the whole java-client part is made differently, but that's another story.*

## Update 
I returned to this task, and I decided to just make 2 simple additions:
1. Java client-side update (Threading, now doesnt affect server and doesnt create lag)
2. More stable server application. (Less errors and uncomfortable mistakes during server running)

This is a friendly mod, so you can do only 1 request to draw a map at the time. 
I dont see any point to make it further, because it's already quite comfortable.
The most annoying things like server lag and rust-server crash are now "gone", so I think
this could be an end to all new features.

Thanks for reading :)
