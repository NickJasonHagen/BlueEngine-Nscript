//rendered
class blueengine{
    func addsquare(id,texture){
        self.square_quee = pooladd(self.square_quee,id);     
    }
    
    func addtexture(fileloc){
        self.texture_quee = pooladd(self.texture_quee,fileloc);
    }
    
    func setpos(object,posx,posy,posz){
        if obj = "" return false
        toset = combine(object,",",posx,",",posy,",",posz)
        self.position_quee = pooladd(self.position_quee,toset);

    }
    func setrot(object,rotation,axis){
        if axis != "x" && axis != "y" &&  axis != "z" {
            return false
        }
        toset = combine(object,",",rotation,",",axis,",")
        self.rotation_quee = pooladd(self.rotation_quee,toset);

    }
    self.square_quee = "" // pool
    self.texture_quee = "" // pool
    for x to 10{
        self.addsquare(combine("square_",x))
        print(combine("quee : square_",x))
    }  
    
    print(self.square_quee)
}
class blueengine_textures{
    // used by engine
    //blueengine_textures = self
}
class sprite{
    func load(){

    }
    func move(){

    }
}
class key{
    //key = "key"
    //keyinput be set to down or up left/right/up/down/e/q/etc by the render engine
}
class player{
    //player = "player"
    self.x = 0
    self.y = 0
    self.z = 0.001
}
blueengine.addtexture("resources/img2.png")
blueengine.addtexture("resources/image.png")
blueengine.addtexture("resources/image.png")// double test checkfail:ok
blueengine.addtexture("resources/player.png")
blueengine.setpos("main",0.0,0.0,0.0)

print(combine("in texture obj=",inobj("blueengine_textures"),"r")
timer = timerinit()
print("starting Nscript")
timer2 = timerinit()
testx = 0.0
testmove = 0.1
fpstimer = timerinit()
fps = 0
debugfps = "false"

coroutine "gameloop" {
    if key.event = "true"{
        if key.left = "down"{
            player.x = math player.x - 0.1
            moved = "true"
        }
        if key.right = "down"{
            player.x = math player.x + 0.1
            moved = "true"
        }
        if key.up = "down"{
            player.y = math player.y + 0.1
            moved = "true"
        }
        if key.down = "down"{
            player.y = math player.y - 0.1
            moved = "true"
        }
        if moved == "true"{
            blueengine.setpos("alt",player.x,player.y,player.z)
            moved = "false"
        }
    }

    fps ++
    if timerdiff(fpstimer) > 999 && debugfps = "true"{
        cwrite(combine("fps=",fps))
        fps = 0
        fpstimer = timerinit()
    }
    if timerdiff(timer) > 19 {
        blueengine = "blueengine"
        testx = math testx + testmove
        blueengine.setrot("main",0.3,"z")
        blueengine.setrot("alt2",0.3,"x")

        timer = timerinit()
    }

}
