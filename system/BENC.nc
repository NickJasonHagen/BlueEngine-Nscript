//rendered
class blueengine{
    func addsquare(id,texture){
        self.square_quee = pooladd(self.square_quee,id);     
    }
    
    func addtexture(fileloc){
        self.texture_q = pooladd(self.texture_q,fileloc);

        return self;
    }
    
    func setposition(object,posx,posy,posz){
        if obj = "" return false
        toset = combine(object,",",posx,",",posy,",",posz)
        self.position_q = pooladd(self.position_q,toset);
    }
    
    func setscale(object,posx,posy,posz){
        if obj = "" return false
        toset = combine(object,",",posx,",",posy,",",posz)
        self.scale_q = pooladd(self.scale_q,toset);

    }

    func setrotation(object,rotation,axis){
        if axis != "x" && axis != "y" &&  axis != "z" {
            return false
        }
        toset = combine(object,",",rotation,",",axis,",")
        self.rotation_q = pooladd(self.rotation_q,toset);

    }
    self.square_q = "" // pool
    self.texture_q = "" // pool
    for x to 10{
        self.addsquare(combine("square_",x))
        print(combine("quee : square_",x))
    }  
    
    print(self.square_q)
    exec(combine(@scriptdir,"system/devtools.nc"))
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
    key = "key"
}

class player{
    //player = "player"
    self.x = 0
    self.y = 0
    self.z = 0.001
    self.sx = 100.0
    self.sy = 100.0
    self.sz = 100.0
}
func test() {

}
textures = ["resources/img2.png","resources/image.png","resources/player.png"]
for x in textures{
    blueengine.addtexture(x)
}

blueengine.setposition("main",0.0,0.0,0.0)

print(combine("in texture obj=",inobj("blueengine_textures"),"r")
timer = timerinit()
print("starting Nscript")
timer2 = timerinit()
testx = 0.0
.add()

testmove = 0.1
fpstimer = timerinit()
fps = 0
debugfps = "false"

coroutine "gameloop" {

    if key.event = "true"{
        if key.a = "down"{
            player.x = math player.x - 0.1
            moved = "true"
        }
        if key.d = "down"{
            player.x = math player.x + 0.1
            moved = "true"
        }
        if key.w = "down"{
            player.y = math player.y + 0.1
            moved = "true"
        }
        if key.s = "down"{
            player.y = math player.y - 0.1
            moved = "true"
        }
        if key.q = "down"{
            player.z = math player.z - 0.1
            moved = "true"
        }
        if key.e = "down"{
            player.z = math player.z + 0.1
            moved = "true"
        }
        if key.p = "down"{
            player.sx = math player.sx + 1
            blueengine.setscale("main",player.sx,player.sy,player.sy)
        }
        if key.o = "down"{
            player.sx = math player.sx - 1
            blueengine.setscale("main",player.sx,player.sy,player.sy)
        }
        if key.x = "down"{
            devtools.runcode()
        }
        if moved == "true"{
            blueengine.setposition("main",player.x,player.y,player.z)
            moved = "false"
        }
    }

    if debugfps == "true"{
        fps ++
        if timerdiff(fpstimer) > 999{
            fps = 0
            fpstimer = timerinit()
        }
        cwrite(combine("fps=",fps))
    }

    if timerdiff(timer) > 19 {
        blueengine = "blueengine"
        testx = math testx + testmove
        //blueengine.setscale("main",random(-5,5,3),random(-5,5,3),0)
        blueengine.setrotation("main",0.9,"z")
        blueengine.setrotation("alt2",0.9,"x")
        blueengine.setrotation("alt2",0.9,"z")
        timer = timerinit()
    }

}

